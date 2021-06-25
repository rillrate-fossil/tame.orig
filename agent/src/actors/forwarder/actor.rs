mod consumer;
mod process_runner;

use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, TaskAddress};
use process_runner::ProcWaiter;
use rillrate_agent_protocol::log_flow::LogFlowTracer;
use rillrate_agent_protocol::process_monitor::{
    Command, ProcessMonitorTracer, ProcessMonitorWatcher,
};

pub struct Forwarder {
    command: Command,
    process_tracer: ProcessMonitorTracer,
    process_watcher: Option<ProcessMonitorWatcher>,
    log_tracer: LogFlowTracer,
    child: Option<TaskAddress<ProcWaiter>>,
}

impl Forwarder {
    pub fn new(command: Command) -> Self {
        let (process_tracer, process_watcher) = ProcessMonitorTracer::new(command.clone());
        let (log_tracer, _log_watcher) = LogFlowTracer::new();
        Self {
            command,
            process_tracer,
            process_watcher: Some(process_watcher),
            log_tracer,
            child: None,
        }
    }
}

impl Actor for Forwarder {
    type GroupBy = ();
}

#[async_trait]
impl StartedBy<Supervisor> for Forwarder {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.listen_to_actions(ctx)?;
        self.spawn_process(ctx);
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for Forwarder {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

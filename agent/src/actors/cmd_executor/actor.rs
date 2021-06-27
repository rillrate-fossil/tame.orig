mod consumer;
mod process_runner;

use crate::actors::supervisor::{Executor, Supervisor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, TaskAddress};
use process_runner::ProcWaiter;
use tame_protocol::cmd::log_flow::LogFlowTracer;
use tame_protocol::cmd::process_monitor::{Command, ProcessMonitorTracer, ProcessMonitorWatcher};

pub struct CmdExecutor {
    command: Command,
    manual: bool,
    process_tracer: ProcessMonitorTracer,
    process_watcher: Option<ProcessMonitorWatcher>,
    log_tracer: LogFlowTracer,
    child: Option<TaskAddress<ProcWaiter>>,
}

impl CmdExecutor {
    pub fn new(command: Command, manual: bool) -> Self {
        let (process_tracer, process_watcher) = ProcessMonitorTracer::new(command.clone());
        let (log_tracer, _log_watcher) = LogFlowTracer::new();
        Self {
            command,
            manual,
            process_tracer,
            process_watcher: Some(process_watcher),
            log_tracer,
            child: None,
        }
    }
}

impl Actor for CmdExecutor {
    type GroupBy = ();
}

impl Executor for CmdExecutor {}

#[async_trait]
impl StartedBy<Supervisor> for CmdExecutor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.consumer(ctx)?;
        if !self.manual {
            self.spawn_process(ctx);
        }
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for CmdExecutor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

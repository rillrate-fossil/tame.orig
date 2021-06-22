mod consumer;
mod process_runner;

use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy};
use rillrate_agent_protocol::process_monitor::tracer::{
    ProcessMonitorTracer, ProcessMonitorWatcher,
};

pub struct Forwarder {
    tracer: ProcessMonitorTracer,
    watcher: Option<ProcessMonitorWatcher>,
}

impl Forwarder {
    pub fn new() -> Self {
        // TODO: Use a parameter here
        let path = "my.logs".parse().unwrap();
        let (tracer, watcher) = ProcessMonitorTracer::new(path);
        Self {
            tracer,
            watcher: Some(watcher),
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
        self.spawn_process(ctx)?;
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

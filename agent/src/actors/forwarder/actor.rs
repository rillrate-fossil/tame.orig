mod proc_runner;

use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Consumer, Context, InterruptedBy, StartedBy};
use rill_protocol::flow::core::{ActionEnvelope, Activity};
use rillrate_agent_protocol::process_monitor::tracer::{
    ProcessMonitorState, ProcessMonitorTracer, ProcessMonitorWatcher,
};
use thiserror::Error;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

#[derive(Debug, Error)]
enum ForwarderError {
    #[error("inner value already taken")]
    AlreadyTaken,
}

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
        // TODO: Spawn a task to read stderr

        let rx = self.watcher.take().ok_or(ForwarderError::AlreadyTaken)?;
        let stream = BroadcastStream::new(rx);
        ctx.attach(stream, (), ());

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

#[async_trait]
impl Consumer<Result<ActionEnvelope<ProcessMonitorState>, BroadcastStreamRecvError>> for Forwarder {
    async fn handle(
        &mut self,
        event: Result<ActionEnvelope<ProcessMonitorState>, BroadcastStreamRecvError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        let envelope = event?;
        match envelope.activity {
            Activity::Connected => {}
            Activity::Action(action) => {}
            Activity::Disconnected => {}
        }
        Ok(())
    }
}

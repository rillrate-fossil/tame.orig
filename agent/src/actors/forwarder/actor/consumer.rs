use super::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Consumer, Context};
use rill_protocol::flow::core::{ActionEnvelope, Activity};
use rillrate_agent_protocol::process_monitor::tracer::ProcessMonitorState;
use thiserror::Error;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

#[derive(Debug, Error)]
#[error("inner value already taken")]
struct AlreadyTaken;

impl Forwarder {
    pub fn listen_to_actions(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let rx = self.watcher.take().ok_or(AlreadyTaken)?;
        let stream = BroadcastStream::new(rx);
        ctx.attach(stream, (), ());
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

use super::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Consumer, Context};
use rill_protocol::flow::core::{ActionEnvelope, Activity};
use tame_protocol::process_monitor::{ProcessMonitorAction, ProcessMonitorState};
use thiserror::Error;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

#[derive(Debug, Error)]
#[error("inner value already taken")]
struct AlreadyTaken;

impl Forwarder {
    pub fn listen_to_actions(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let rx = self.process_watcher.take().ok_or(AlreadyTaken)?;
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
        ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        let envelope = event?;
        match envelope.activity {
            Activity::Connected => {}
            Activity::Action(action) => match action {
                ProcessMonitorAction::Kill => {
                    self.kill_process()?;
                }
                ProcessMonitorAction::Respawn => {
                    self.spawn_process(ctx);
                }
            },
            Activity::Disconnected => {}
        }
        Ok(())
    }
}

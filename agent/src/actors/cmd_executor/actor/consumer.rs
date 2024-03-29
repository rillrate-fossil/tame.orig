use super::CmdExecutor;
use crate::actors::error::AlreadyTaken;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Consumer, Context};
use rill_protocol::flow::core::{ActionEnvelope, Activity};
use tame_protocol::cmd::process_monitor::{ProcessMonitorAction, ProcessMonitorState};
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

impl CmdExecutor {
    pub fn consumer(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let rx = self.process_watcher.take().ok_or(AlreadyTaken)?;
        let stream = BroadcastStream::new(rx);
        ctx.attach(stream, (), ());
        Ok(())
    }
}

#[async_trait]
impl Consumer<Result<ActionEnvelope<ProcessMonitorState>, BroadcastStreamRecvError>>
    for CmdExecutor
{
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

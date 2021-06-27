use super::TopExecutor;
use crate::actors::error::AlreadyTaken;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Consumer, Context};
use rill_protocol::flow::core::{ActionEnvelope, Activity};
use tame_protocol::top::process_list::{ProcessListAction, ProcessListState};
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

impl TopExecutor {
    pub fn consumer(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let rx = self.process_watcher.take().ok_or(AlreadyTaken)?;
        let stream = BroadcastStream::new(rx);
        ctx.attach(stream, (), ());
        Ok(())
    }
}

#[async_trait]
impl Consumer<Result<ActionEnvelope<ProcessListState>, BroadcastStreamRecvError>> for TopExecutor {
    async fn handle(
        &mut self,
        event: Result<ActionEnvelope<ProcessListState>, BroadcastStreamRecvError>,
        ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        let envelope = event?;
        match envelope.activity {
            Activity::Connected => {
                self.listeners.lock().await.insert(envelope.origin, ());
                // TODO: Send the default mode directly
            }
            Activity::Action(action) => {
                // TODO: Switch mode?
            }
            Activity::Disconnected => {
                self.listeners.lock().await.remove(&envelope.origin);
            }
        }
        Ok(())
    }
}

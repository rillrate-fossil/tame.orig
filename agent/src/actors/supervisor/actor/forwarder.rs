use super::{Group, Supervisor, SupervisorLink};
use crate::actors::forwarder::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Action, ActionHandler, Context, Eliminated, IdOf};
use rillrate_agent_protocol::process_monitor::Command;

pub struct SpawnCommand {
    command: Command,
}

impl Action for SpawnCommand {}

impl SupervisorLink {
    pub async fn spawn_command(&mut self, command: Command) -> Result<(), Error> {
        let msg = SpawnCommand { command };
        self.address.act(msg).await
    }
}

#[async_trait]
impl ActionHandler<SpawnCommand> for Supervisor {
    async fn handle(&mut self, msg: SpawnCommand, ctx: &mut Context<Self>) -> Result<(), Error> {
        let worker = Forwarder::new(msg.command);
        ctx.spawn_actor(worker, Group::Workers);
        Ok(())
    }
}

#[async_trait]
impl Eliminated<Forwarder> for Supervisor {
    async fn handle(
        &mut self,
        _id: IdOf<Forwarder>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

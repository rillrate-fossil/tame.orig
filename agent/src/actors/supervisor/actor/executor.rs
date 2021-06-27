use super::{Group, Supervisor, SupervisorLink};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Action, ActionHandler, Actor, Context, Eliminated, IdOf, InterruptedBy, StartedBy};

pub trait Executor: Actor + StartedBy<Supervisor> + InterruptedBy<Supervisor> {}

pub struct AttachExecutor<T: Executor> {
    actor: T,
}

impl<T: Executor> Action for AttachExecutor<T> {}

impl SupervisorLink {
    pub async fn spawn_executor<T: Executor>(&mut self, actor: T) -> Result<(), Error> {
        let msg = AttachExecutor { actor };
        self.address.act(msg).await
    }
}

#[async_trait]
impl<T: Executor> ActionHandler<AttachExecutor<T>> for Supervisor {
    async fn handle(
        &mut self,
        msg: AttachExecutor<T>,
        ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        ctx.spawn_actor(msg.actor, Group::Workers);
        Ok(())
    }
}

#[async_trait]
impl<T: Executor> Eliminated<T> for Supervisor {
    async fn handle(&mut self, _id: IdOf<T>, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }
}

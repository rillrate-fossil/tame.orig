use super::{Group, Supervisor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, Eliminated, IdOf};
use rill_engine::RillEngine;

impl Supervisor {
    pub fn spawn_engine(&mut self, ctx: &mut Context<Self>) {
        let engine = RillEngine::new(self.config.clone());
        ctx.spawn_actor(engine, Group::Engine);
    }
}

#[async_trait]
impl Eliminated<RillEngine> for Supervisor {
    async fn handle(
        &mut self,
        _id: IdOf<RillEngine>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

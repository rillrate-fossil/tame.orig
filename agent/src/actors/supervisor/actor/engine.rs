use super::{Group, Supervisor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, Eliminated, IdOf};
use rill_engine::{EngineConfig, RillEngine};
use rillrate_agent_protocol::provider_type;

impl Supervisor {
    pub fn spawn_engine(&mut self, ctx: &mut Context<Self>) {
        let engine = RillEngine::new(EngineConfig::new(provider_type()));
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

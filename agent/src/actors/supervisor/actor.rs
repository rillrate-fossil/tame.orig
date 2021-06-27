mod engine;
pub mod executor;

use anyhow::Error;
use async_trait::async_trait;
use derive_more::From;
use meio::{Actor, Address, Context, InterruptedBy, StartedBy, System};
use rill_engine::EngineConfig;

#[derive(Debug, Clone, From)]
pub struct SupervisorLink {
    address: Address<Supervisor>,
}

pub struct Supervisor {
    config: EngineConfig,
}

impl Supervisor {
    pub fn new(config: EngineConfig) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Group {
    Engine,
    Workers,
}

impl Actor for Supervisor {
    type GroupBy = Group;
}

#[async_trait]
impl StartedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.spawn_engine(ctx);
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<System> for Supervisor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

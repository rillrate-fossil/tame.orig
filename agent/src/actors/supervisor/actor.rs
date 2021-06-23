mod engine;
mod forwarder;

use anyhow::Error;
use async_trait::async_trait;
use derive_more::From;
use meio::{Actor, Address, Context, InterruptedBy, StartedBy, System};

#[derive(Debug, Clone, From)]
pub struct SupervisorLink {
    address: Address<Supervisor>,
}

pub struct Supervisor {}

impl Supervisor {
    pub fn new() -> Self {
        Self {}
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

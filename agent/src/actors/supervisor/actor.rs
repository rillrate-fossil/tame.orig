use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, Eliminated, IdOf, InterruptedBy, StartedBy, System};
use rill_engine::{EngineConfig, RillEngine};
use rillrate_logs_protocol::provider_type;

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
        let engine = RillEngine::new(EngineConfig::new(provider_type()));
        ctx.spawn_actor(engine, Group::Engine);

        /*
        let worker = Worker::new();
        ctx.spawn_actor(worker, Group::Workers);
        */

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

/*
#[async_trait]
impl Eliminated<Worker> for Supervisor {
    async fn handle(&mut self, _id: IdOf<Worker>, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }
}
*/

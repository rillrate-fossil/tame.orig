mod consumer;

use crate::actors::supervisor::{Executor, Supervisor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, TaskAddress};
use tame_protocol::top::process_list::{ProcessListTracer, ProcessListWatcher};

pub struct TopExecutor {
    process_tracer: ProcessListTracer,
    process_watcher: Option<ProcessListWatcher>,
}

impl Actor for TopExecutor {
    type GroupBy = ();
}

impl Executor for TopExecutor {}

#[async_trait]
impl StartedBy<Supervisor> for TopExecutor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        self.consumer(ctx)?;
        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for TopExecutor {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

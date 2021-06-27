mod consumer;

use crate::actors::supervisor::{Executor, Supervisor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, TaskAddress};
use rill_protocol::io::provider::ProviderReqId;
use std::collections::HashMap;
use tame_protocol::top::process_list::{ProcessListTracer, ProcessListWatcher};

pub struct TopExecutor {
    listeners: HashMap<ProviderReqId, ()>,
    process_tracer: ProcessListTracer,
    process_watcher: Option<ProcessListWatcher>,
}

impl TopExecutor {
    pub fn new() -> Self {
        let (process_tracer, process_watcher) = ProcessListTracer::new();
        Self {
            listeners: HashMap::new(),
            process_tracer,
            process_watcher: Some(process_watcher),
        }
    }
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

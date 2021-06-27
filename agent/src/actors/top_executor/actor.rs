mod consumer;
mod process_tracker;

use crate::actors::supervisor::{Executor, Supervisor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Context, InterruptedBy, StartedBy, TaskAddress};
use rill_protocol::io::provider::ProviderReqId;
use std::collections::HashMap;
use std::sync::Arc;
use tame_protocol::top::process_list::{ProcessListTracer, ProcessListWatcher};
use tokio::sync::Mutex;

pub type ListenersMap = Arc<Mutex<HashMap<ProviderReqId, ()>>>;

pub struct TopExecutor {
    listeners: ListenersMap,
    process_tracer: ProcessListTracer,
    process_watcher: Option<ProcessListWatcher>,
}

impl TopExecutor {
    pub fn new() -> Self {
        let (process_tracer, process_watcher) = ProcessListTracer::new();
        Self {
            listeners: Arc::new(Mutex::new(HashMap::new())),
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
        self.spawn_tracker(ctx);
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

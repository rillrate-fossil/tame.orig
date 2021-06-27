use super::{ListenersMap, TopExecutor};
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use std::collections::HashMap;
use sysinfo::{ProcessExt, System, SystemExt};
use tame_protocol::top::process_list::{ProcessListTracer, ProcessRecord};

impl TopExecutor {
    pub fn spawn_tracker(&mut self, ctx: &mut Context<Self>) {
        let tracker = ProcessTracker {
            process_tracer: self.process_tracer.clone(),
            system: System::new_all(),
        };
        ctx.spawn_task(tracker, (), ());
    }
}

pub struct ProcessTracker {
    process_tracer: ProcessListTracer,
    system: System,
}

#[async_trait]
impl LiteTask for ProcessTracker {
    type Output = ();
    async fn repeatable_routine(&mut self) -> Result<Option<Self::Output>, Error> {
        self.system.refresh_all();
        let mut snapshot = HashMap::new();
        for (pid, proc) in self.system.get_processes().iter().take(20) {
            let info = ProcessRecord {
                name: proc.name().to_string(),
            };
            snapshot.insert(*pid, info);
        }
        self.process_tracer.snapshot(snapshot.clone());
        Ok(None)
    }
}

#[async_trait]
impl TaskEliminated<ProcessTracker, ()> for TopExecutor {
    async fn handle(
        &mut self,
        _id: IdOf<ProcessTracker>,
        _tag: (),
        _res: Result<(), TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        // TODO: Set exit status of the process (with a tracer)
        Ok(())
    }
}

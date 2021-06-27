use super::TopExecutor;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use sysinfo::{ProcessExt, System, SystemExt};

impl TopExecutor {
    pub fn spawn_tracker(&mut self, ctx: &mut Context<Self>) {
        let tracker = ProcessTracker {
            system: System::new_all(),
        };
        ctx.spawn_task(tracker, (), ());
    }
}

pub struct ProcessTracker {
    system: System,
}

#[async_trait]
impl LiteTask for ProcessTracker {
    type Output = ();
    async fn repeatable_routine(&mut self) -> Result<Option<Self::Output>, Error> {
        self.system.refresh_all();
        for (pid, proc_) in self.system.get_processes() {}
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

pub use super::flow::*;
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::{Tracer, Watcher};
use rill_protocol::io::provider::ProviderReqId;
use std::collections::HashMap;

pub type ProcessListWatcher = Watcher<ProcessListState>;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct ProcessListTracer {
    tracer: Tracer<ProcessListState>,
}

impl ProcessListTracer {
    pub fn new() -> (Self, ProcessListWatcher) {
        let state = ProcessListState::new();
        let (tracer, watcher) = Tracer::new_push(state, LOCATION.root());
        (Self { tracer }, watcher)
    }

    pub fn snapshot(&self, snapshot: HashMap<Pid, ProcessRecord>) {
        let msg = ProcessListEvent::UpdateSnapshot { snapshot };
        self.tracer.send(msg, None, None);
    }
}

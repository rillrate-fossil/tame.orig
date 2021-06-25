pub use super::flow::*;
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::{Tracer, Watcher};

pub type LogFlowWatcher = Watcher<LogFlowState>;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct LogFlowTracer {
    tracer: Tracer<LogFlowState>,
}

impl LogFlowTracer {
    pub fn new() -> (Self, LogFlowWatcher) {
        let state = LogFlowState::new();
        let (tracer, watcher) = Tracer::new_push(state, LOCATION.root());
        (Self { tracer }, watcher)
    }

    pub fn add_logs(&self, lines: Vec<String>) {
        let msg = LogFlowEvent::AddLogs { lines };
        self.tracer.send(msg, None, None);
    }

    pub fn clear_logs(&self) {
        let msg = LogFlowEvent::ClearLogs;
        self.tracer.send(msg, None, None);
    }
}

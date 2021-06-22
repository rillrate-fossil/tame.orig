pub use super::flow::*;
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::{Tracer, Watcher};
use rill_protocol::io::provider::Path;

pub type ProcessMonitorWatcher = Watcher<ProcessMonitorState>;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct ProcessMonitorTracer {
    tracer: Tracer<ProcessMonitorState>,
}

impl ProcessMonitorTracer {
    pub fn new(path: Path) -> (Self, ProcessMonitorWatcher) {
        let state = ProcessMonitorState::new();
        let (tracer, watcher) = Tracer::new_push(state, path);
        (Self { tracer }, watcher)
    }
}

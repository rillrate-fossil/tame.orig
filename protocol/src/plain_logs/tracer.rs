pub use super::flow::*;
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::{Tracer, Watcher};
use rill_protocol::io::provider::Path;

pub type PlainLogsWatcher = Watcher<PlainLogsState>;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PlainLogsTracer {
    tracer: Tracer<PlainLogsState>,
}

impl PlainLogsTracer {
    pub fn new(path: Path) -> (Self, PlainLogsWatcher) {
        let state = PlainLogsState::new();
        let (tracer, watcher) = Tracer::new_push(state, path);
        (Self { tracer }, watcher)
    }
}

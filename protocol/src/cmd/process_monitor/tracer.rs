pub use super::flow::*;
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::{Tracer, Watcher};

pub type ProcessMonitorWatcher = Watcher<ProcessMonitorState>;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct ProcessMonitorTracer {
    tracer: Tracer<ProcessMonitorState>,
}

impl ProcessMonitorTracer {
    pub fn new(command: Command) -> (Self, ProcessMonitorWatcher) {
        let state = ProcessMonitorState::new(command);
        let (tracer, watcher) = Tracer::new_push(state, LOCATION.root());
        (Self { tracer }, watcher)
    }

    pub fn assign_pid(&self, pid: Option<Pid>) {
        let msg = ProcessMonitorEvent::AssignPid { pid };
        self.tracer.send(msg, None, None);
    }

    pub fn set_exit_code(&self, code: Option<ExitCode>) {
        let msg = ProcessMonitorEvent::SetExitCode { code };
        self.tracer.send(msg, None, None);
    }
}

use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};

pub const LOCATION: Location = Location::new("system:process_monitor");

pub type Pid = u32;
pub type ExitCode = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    pub workdir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessStatus {
    NotDetected,
    Alive { pid: Option<Pid> },
    Terminated { code: Option<ExitCode> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMonitorState {
    pub command: Command,
    pub process_status: ProcessStatus,
}

#[allow(clippy::new_without_default)]
impl ProcessMonitorState {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            process_status: ProcessStatus::NotDetected,
        }
    }
}

impl Flow for ProcessMonitorState {
    type Action = ProcessMonitorAction;
    type Event = ProcessMonitorEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::tame::cmd::process_monitor::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            ProcessMonitorEvent::AssignPid { pid } => {
                self.process_status = ProcessStatus::Alive { pid };
            }
            ProcessMonitorEvent::SetExitCode { code } => {
                self.process_status = ProcessStatus::Terminated { code };
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessMonitorAction {
    /// This action is available for terminated processes only
    Respawn,
    /// This action is available for alive processes only
    Kill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessMonitorEvent {
    AssignPid { pid: Option<Pid> },
    SetExitCode { code: Option<ExitCode> },
}

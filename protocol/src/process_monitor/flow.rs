use rill_protocol::flow::core::{Flow, TimedEvent};
//use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub type Pid = u32;
pub type ExitCode = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessStatus {
    NotDetected,
    Alive { pid: Option<Pid> },
    Terminated { code: Option<ExitCode> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMonitorState {
    command: Command,
    process_status: ProcessStatus,
    depth: u32,
    logs: VecDeque<String>,
}

#[allow(clippy::new_without_default)]
impl ProcessMonitorState {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            process_status: ProcessStatus::NotDetected,
            depth: 128,
            logs: VecDeque::new(),
        }
    }
}

impl Flow for ProcessMonitorState {
    type Action = ProcessMonitorAction;
    type Event = ProcessMonitorEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::agent::process_monitor::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            ProcessMonitorEvent::AddLogs { lines } => {
                self.logs.extend(lines.into_iter());
                let len = self.logs.len();
                let depth = self.depth as usize;
                if len > depth {
                    let diff = len - depth;
                    drop(self.logs.drain(0..diff));
                }
            }
            ProcessMonitorEvent::ClearLogs => {
                self.logs.clear();
            }
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
    AddLogs { lines: Vec<String> },
    ClearLogs,
    AssignPid { pid: Option<Pid> },
    SetExitCode { code: Option<ExitCode> },
}

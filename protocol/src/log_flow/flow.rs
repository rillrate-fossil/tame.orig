use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const LOCATION: Location = Location::new("system:log_flow");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFlow {
    pub depth: u32,
    pub logs: VecDeque<String>,
}

#[allow(clippy::new_without_default)]
impl LogFlow {
    pub fn new() -> Self {
        Self {
            depth: 128,
            logs: VecDeque::new(),
        }
    }
}

impl Flow for LogFlow {
    type Action = LogFlowAction;
    type Event = LogFlowEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::agent::log_flow::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            LogFlowEvent::AddLogs { lines } => {
                self.logs.extend(lines.into_iter());
                let len = self.logs.len();
                let depth = self.depth as usize;
                if len > depth {
                    let diff = len - depth;
                    drop(self.logs.drain(0..diff));
                }
            }
            LogFlowEvent::ClearLogs => {
                self.logs.clear();
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFlowAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFlowEvent {
    AddLogs { lines: Vec<String> },
    ClearLogs,
}

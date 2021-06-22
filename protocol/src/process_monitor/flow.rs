use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMonitorState {
    logs: VecDeque<String>,
}

#[allow(clippy::new_without_default)]
impl ProcessMonitorState {
    pub fn new() -> Self {
        Self {
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

    fn apply(&mut self, event: TimedEvent<Self::Event>) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessMonitorAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessMonitorEvent {
    AddLogs { lines: Vec<String> },
    CleanLogs,
}

use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const LOCATION: Location = Location::new("system:process_list");

pub type Pid = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRecord {
    /// Name of a process.
    pub name: String,
    pub pid: Pid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessListState {
    #[serde(with = "vectorize")]
    pub snapshot: HashMap<Pid, ProcessRecord>,
}

#[allow(clippy::new_without_default)]
impl ProcessListState {
    pub fn new() -> Self {
        Self {
            snapshot: HashMap::new(),
        }
    }
}

impl Flow for ProcessListState {
    type Action = ProcessListAction;
    type Event = ProcessListEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::tame::cmd::process_list::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            ProcessListEvent::UpdateSnapshot { snapshot } => {
                self.snapshot = snapshot;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessListAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessListEvent {
    UpdateSnapshot {
        #[serde(with = "vectorize")]
        snapshot: HashMap<Pid, ProcessRecord>,
    },
}

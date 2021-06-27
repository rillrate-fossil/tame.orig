use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};

pub const LOCATION: Location = Location::new("system:process_list");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRecord {
    /// Name of a process.
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessListState {
    pub snapshot: Option<Vec<ProcessRecord>>,
}

#[allow(clippy::new_without_default)]
impl ProcessListState {
    pub fn new() -> Self {
        Self { snapshot: None }
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
            ProcessListEvent::Snapshot { snapshot } => {
                self.snapshot = Some(snapshot);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessListAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessListEvent {
    Snapshot { snapshot: Vec<ProcessRecord> },
}

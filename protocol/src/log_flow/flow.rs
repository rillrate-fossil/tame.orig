use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};

pub const LOCATION: Location = Location::new("system:log_flow");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFlow {}

#[allow(clippy::new_without_default)]
impl LogFlow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Flow for LogFlow {
    type Action = LogFlowAction;
    type Event = LogFlowEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::agent::log_flow::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFlowAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFlowEvent {}

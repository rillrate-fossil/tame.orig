use rill_protocol::flow::core::{Flow, TimedEvent};
use rill_protocol::flow::location::Location;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//pub const LOCATION: Location = Location::new("logs:plain_logs");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainLogsState {}

#[allow(clippy::new_without_default)]
impl PlainLogsState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Flow for PlainLogsState {
    type Action = PlainLogsAction;
    type Event = PlainLogsEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate::logs::plain_logs::v0")
    }

    fn apply(&mut self, event: TimedEvent<Self::Event>) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlainLogsAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlainLogsEvent {}

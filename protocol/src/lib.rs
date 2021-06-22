use rill_protocol::io::provider::StreamType;

pub fn provider_type() -> StreamType {
    "rillrate::agent::logs".into()
}

pub mod process_monitor;

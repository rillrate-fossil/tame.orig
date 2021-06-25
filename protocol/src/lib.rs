use once_cell::sync::Lazy;
use rill_protocol::io::provider::StreamType;

pub fn provider_type() -> StreamType {
    "rillrate::agent".into()
}

pub const TYPE: Lazy<StreamType> = Lazy::new(provider_type);

pub mod log_flow;
pub mod process_monitor;

use anyhow::Error;
use meio::System;
use rillrate_agent::actors::supervisor::{Supervisor, SupervisorLink};
use rillrate_agent_protocol::process_monitor::tracer::Command;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    log::info!("Starting RillRate Agent...");
    let sup = Supervisor::new();
    let addr = System::spawn(sup);
    let mut link: SupervisorLink = addr.link();
    let command = Command {
        command: "cat".into(),
        args: Vec::new(),
    };
    link.spawn_command(command).await?;
    System::wait_or_interrupt(addr).await?;
    Ok(())
}

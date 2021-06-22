use anyhow::Error;
use meio::System;
use rillrate_agent::actors::supervisor::Supervisor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    log::info!("Starting RillRate Agent...");
    let sup = Supervisor::new();
    System::spawn_and_wait(sup).await;
    Ok(())
}

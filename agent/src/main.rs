use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    Ok(())
}

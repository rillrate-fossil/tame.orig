use anyhow::Error;
use async_trait::async_trait;
use meio::LiteTask;
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub struct ProcRunner {
    command: String,
    args: Vec<String>,
}

#[async_trait]
impl LiteTask for ProcRunner {
    type Output = ExitStatus;

    async fn interruptable_routine(self) -> Result<Self::Output, Error> {
        let mut child = Command::new(self.command)
            .args(self.args)
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        if let Some(stderr) = child.stderr.take() {
            let lines = BufReader::new(stderr).lines();
        }
        let status = child.wait().await?;
        Ok(status)
    }
}

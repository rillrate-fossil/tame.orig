use super::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::StreamExt;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use rillrate_agent_protocol::process_monitor::tracer::ProcessMonitorTracer;
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio_stream::wrappers::LinesStream;

impl Forwarder {
    pub fn spawn_process(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let runner = ProcessRunner {
            tracer: self.tracer.clone(),
            command: "".into(),
            args: Vec::new(),
        };
        ctx.spawn_task(runner, (), ());
        Ok(())
    }
}

pub struct ProcessRunner {
    tracer: ProcessMonitorTracer,
    command: String,
    args: Vec<String>,
}

#[async_trait]
impl LiteTask for ProcessRunner {
    type Output = ExitStatus;

    async fn interruptable_routine(self) -> Result<Self::Output, Error> {
        self.tracer.clear_logs();
        let mut child = Command::new(self.command)
            .args(self.args)
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        if let Some(stderr) = child.stderr.take() {
            let lines = BufReader::new(stderr).lines();
            let mut chunks = LinesStream::new(lines).ready_chunks(64);
            while let Some(lines) = chunks.next().await {
                let res: Result<Vec<_>, _> = lines.into_iter().collect();
                self.tracer.add_logs(res?);
            }
        }
        let status = child.wait().await?;
        Ok(status)
    }
}

#[async_trait]
impl TaskEliminated<ProcessRunner, ()> for Forwarder {
    async fn handle(
        &mut self,
        _id: IdOf<ProcessRunner>,
        _tag: (),
        _res: Result<ExitStatus, TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        // TODO: Set exit status of the process (with a tracer)
        Ok(())
    }
}

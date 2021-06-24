use super::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::StreamExt;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use rillrate_agent_protocol::process_monitor::tracer::{Command, ProcessMonitorTracer};
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio_stream::wrappers::LinesStream;

impl Forwarder {
    pub fn spawn_process(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let runner = LogReader {
            tracer: self.tracer.clone(),
            command: self.command.clone(),
        };
        ctx.spawn_task(runner, (), ());
        Ok(())
    }
}

pub struct LogReader {
    tracer: ProcessMonitorTracer,
    command: Command,
}

#[async_trait]
impl LiteTask for LogReader {
    type Output = ExitStatus;

    async fn interruptable_routine(self) -> Result<Self::Output, Error> {
        self.tracer.clear_logs();
        let mut child = TokioCommand::new(self.command.command)
            .args(self.command.args)
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        self.tracer.assign_pid(child.id());
        if let Some(stderr) = child.stderr.take() {
            let lines = BufReader::new(stderr).lines();
            let mut chunks = LinesStream::new(lines).ready_chunks(64);
            while let Some(lines) = chunks.next().await {
                let res: Result<Vec<_>, _> = lines.into_iter().collect();
                self.tracer.add_logs(res?);
            }
        }
        let status = child.wait().await?;
        self.tracer.set_exit_code(status.code());
        Ok(status)
    }
}

#[async_trait]
impl TaskEliminated<LogReader, ()> for Forwarder {
    async fn handle(
        &mut self,
        _id: IdOf<LogReader>,
        _tag: (),
        _res: Result<ExitStatus, TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        // TODO: Set exit status of the process (with a tracer)
        Ok(())
    }
}

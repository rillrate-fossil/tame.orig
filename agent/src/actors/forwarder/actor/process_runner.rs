use super::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::StreamExt;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use rillrate_agent_protocol::process_monitor::tracer::{Command, ProcessMonitorTracer};
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStderr, Command as TokioCommand};
use tokio_stream::wrappers::LinesStream;

impl Forwarder {
    pub fn spawn_process(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        if self.child.is_none() {
            self.tracer.clear_logs();
            let Command { command, args } = self.command.clone();
            let mut child = TokioCommand::new(command)
                .args(args)
                .stderr(Stdio::piped())
                .kill_on_drop(true)
                .spawn()?;
            self.tracer.assign_pid(child.id());

            if let Some(stderr) = child.stderr.take() {
                let runner = LogReader {
                    tracer: self.tracer.clone(),
                    stderr,
                };
                ctx.spawn_task(runner, (), ());
            }

            self.child = Some(child);

            Ok(())
        } else {
            Err(Error::msg("Already spawned"))
        }
    }

    pub fn kill_process(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        if let Some(child) = self.child.take() {
            let killer = ProcKiller { child };
            ctx.spawn_task(killer, (), ());
            Ok(())
        } else {
            Err(Error::msg("Not implemented"))
        }
    }
}

pub struct ProcKiller {
    child: Child,
}

#[async_trait]
impl LiteTask for ProcKiller {
    type Output = ExitStatus;
    async fn interruptable_routine(mut self) -> Result<Self::Output, Error> {
        // TODO: Use alternative ways to interrupt it
        self.child.kill().await?;
        let status = self.child.wait().await?;
        Ok(status)
    }
}

#[async_trait]
impl TaskEliminated<ProcKiller, ()> for Forwarder {
    async fn handle(
        &mut self,
        _id: IdOf<ProcKiller>,
        _tag: (),
        res: Result<ExitStatus, TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        match res {
            Ok(status) => {
                self.tracer.set_exit_code(status.code());
                Ok(())
            }
            Err(err) => {
                self.tracer.set_exit_code(None);
                Err(err.into())
            }
        }
    }
}

pub struct LogReader {
    tracer: ProcessMonitorTracer,
    stderr: ChildStderr,
}

#[async_trait]
impl LiteTask for LogReader {
    type Output = ();

    async fn interruptable_routine(self) -> Result<Self::Output, Error> {
        let lines = BufReader::new(self.stderr).lines();
        let mut chunks = LinesStream::new(lines).ready_chunks(64);
        while let Some(lines) = chunks.next().await {
            let res: Result<Vec<_>, _> = lines.into_iter().collect();
            self.tracer.add_logs(res?);
        }
        Ok(())
    }
}

#[async_trait]
impl TaskEliminated<LogReader, ()> for Forwarder {
    async fn handle(
        &mut self,
        _id: IdOf<LogReader>,
        _tag: (),
        _res: Result<(), TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        // TODO: Set exit status of the process (with a tracer)
        Ok(())
    }
}

use super::Forwarder;
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::StreamExt;
use meio::{Context, IdOf, LiteTask, StopReceiver, TaskEliminated, TaskError};
use rillrate_agent_protocol::process_monitor::tracer::{Command, ProcessMonitorTracer};
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStderr, Command as TokioCommand};
use tokio_stream::wrappers::LinesStream;

impl Forwarder {
    pub fn spawn_process(&mut self, ctx: &mut Context<Self>) {
        if self.child.is_none() {
            self.tracer.clear_logs();
            let Command {
                command,
                args,
                workdir,
            } = self.command.clone();
            let child = TokioCommand::new(command)
                .args(args)
                .current_dir(workdir)
                .stderr(Stdio::piped())
                .kill_on_drop(true)
                .spawn();
            match child {
                Ok(mut child) => {
                    self.tracer.assign_pid(child.id());

                    if let Some(stderr) = child.stderr.take() {
                        let runner = LogReader {
                            tracer: self.tracer.clone(),
                            stderr,
                        };
                        ctx.spawn_task(runner, (), ());
                    }

                    let killer = ProcWaiter { child };
                    let addr = ctx.spawn_task(killer, (), ());
                    self.child = Some(addr);
                }
                Err(err) => {
                    log::error!("Can't spawn a process: {}", err);
                    self.tracer.set_exit_code(None);
                }
            }
        } else {
            log::error!("Attempt to spawn process twice");
        }
    }

    pub fn kill_process(&mut self) -> Result<(), Error> {
        if let Some(child) = self.child.as_ref() {
            child.stop()?;
            Ok(())
        } else {
            Err(Error::msg("Not alive process"))
        }
    }
}

pub struct ProcWaiter {
    child: Child,
}

#[async_trait]
impl LiteTask for ProcWaiter {
    type Output = ExitStatus;
    async fn routine(mut self, mut stop: StopReceiver) -> Result<Self::Output, Error> {
        let res = stop.or(self.child.wait()).await;
        match res {
            Ok(exit_status) => Ok(exit_status?),
            Err(err) => {
                self.child.kill().await?;
                Err(err.into())
            }
        }
    }
}

#[async_trait]
impl TaskEliminated<ProcWaiter, ()> for Forwarder {
    async fn handle(
        &mut self,
        _id: IdOf<ProcWaiter>,
        _tag: (),
        res: Result<ExitStatus, TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        self.child.take();
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

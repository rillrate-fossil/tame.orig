mod opts;

use anyhow::Error;
use clap::Clap;
use meio::System;
use opts::{Opts, SubCommand};
use rill_engine::EngineConfig;
use rill_protocol::io::provider::EntryId;
use std::env;
use tame::actors::supervisor::{Supervisor, SupervisorLink};
use tame_protocol::cmd::process_monitor::Command;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    env_logger::try_init()?;
    log::info!("Starting RillRate Agent...");
    let addr;
    match opts.subcmd {
        SubCommand::Cmd(cmd) => {
            use tame::actors::cmd_executor::CmdExecutor;
            use tame_protocol::cmd::provider_type;
            let command = extract_command(cmd.command)?;
            let mut config = EngineConfig::new(provider_type());
            if let Some(name) = opts.name {
                config.name = Some(name);
            } else {
                config.name = Some(smart_name(&command.command));
            }
            let sup = Supervisor::new(config);
            addr = System::spawn(sup);
            let mut link: SupervisorLink = addr.link();
            let executor = CmdExecutor::new(command, cmd.no_spawn);
            link.spawn_executor(executor).await?;
        }
        SubCommand::Top(top) => {
            use tame::actors::top_executor::TopExecutor;
            use tame_protocol::top::provider_type;
            let mut config = EngineConfig::new(provider_type());
            if let Some(name) = opts.name {
                config.name = Some(name);
            }
            let sup = Supervisor::new(config);
            addr = System::spawn(sup);
            let mut link: SupervisorLink = addr.link();
            let executor = TopExecutor::new();
            link.spawn_executor(executor).await?;
        }
    }
    System::wait_or_interrupt(addr).await?;
    Ok(())
}

fn smart_name(name: &str) -> EntryId {
    enum State {
        WaitAlpha,
        TakeAlphas,
    }
    let mut state = State::WaitAlpha;
    let mut s = String::new();
    for c in name.chars() {
        match state {
            State::WaitAlpha => {
                if c.is_alphanumeric() {
                    if !s.is_empty() {
                        s.push('-');
                    }
                    s.push(c);
                    state = State::TakeAlphas;
                }
            }
            State::TakeAlphas => {
                if c.is_alphanumeric() {
                    s.push(c);
                } else {
                    state = State::WaitAlpha;
                }
            }
        }
    }
    s.into()
}

fn extract_command(args: Vec<String>) -> Result<Command, Error> {
    // TODO: Provide an option to set work dir
    let workdir = env::current_dir()?.as_path().to_string_lossy().to_string();
    let mut input = args.into_iter();
    let command = input.next();
    let args = input.collect();
    if let Some(command) = command {
        Ok(Command {
            command,
            args,
            workdir,
        })
    } else {
        Err(Error::msg("No command provided"))
    }
}

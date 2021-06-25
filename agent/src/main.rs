use anyhow::Error;
use meio::System;
use rill_engine::EngineConfig;
use rillrate_agent::actors::supervisor::{Supervisor, SupervisorLink};
use rillrate_agent_protocol::process_monitor::tracer::Command;
use rillrate_agent_protocol::provider_type;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let command = extract_command()?;
    log::info!("Starting RillRate Agent...");
    let mut config = EngineConfig::new(provider_type());
    config.name = Some(smart_name(&command.command).into());
    let sup = Supervisor::new(config);
    let addr = System::spawn(sup);
    let mut link: SupervisorLink = addr.link();
    link.spawn_command(command).await?;
    System::wait_or_interrupt(addr).await?;
    Ok(())
}

fn smart_name(name: &str) -> String {
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
    s
}

fn extract_command() -> Result<Command, Error> {
    // TODO: Provide an option to set work dir
    let workdir = env::current_dir()?.as_path().to_string_lossy().to_string();
    let mut input = env::args();
    let mut command = None;
    let mut args = Vec::new();
    while let Some(arg) = input.next() {
        if arg == "--" {
            command = input.next();
            args.extend(input);
            break;
        }
    }
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

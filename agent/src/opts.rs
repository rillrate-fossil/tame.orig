use clap::{AppSettings, ArgSettings, Clap};
use rill_protocol::io::provider::EntryId;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, about = "Name of the provider")]
    pub name: Option<EntryId>,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about = "Spawn and track a command")]
    Cmd(CmdCommand),
    #[clap(about = "Track process of the system")]
    Top(TopCommand),
}

#[derive(Clap)]
pub struct CmdCommand {
    #[clap(setting = ArgSettings::Last, about = "The command with arguments to spawn")]
    pub command: Vec<String>,
    #[clap(long, about = "No spawn command immediately")]
    pub no_spawn: bool,
}

#[derive(Clap)]
pub struct TopCommand {}

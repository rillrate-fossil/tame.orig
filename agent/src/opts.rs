use clap::{AppSettings, ArgSettings, Clap};
use rill_protocol::io::provider::EntryId;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, about = "Name of the provider")]
    pub name: Option<EntryId>,
    #[clap(setting = ArgSettings::Last, about = "The command with arguments to spawn")]
    pub command: Vec<String>,
    #[clap(long, about = "No spawn command immediately")]
    pub no_spawn: bool,
}

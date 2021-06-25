use clap::{AppSettings, ArgSettings, Clap};
use rill_protocol::io::provider::EntryId;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    pub name: Option<EntryId>,
    #[clap(setting = ArgSettings::Last)]
    pub command: Vec<String>,
}

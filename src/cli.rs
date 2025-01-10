
use std::path::PathBuf;

use clap::{command, Command, Parser, Subcommand};



#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ElfDump {
    pub name: Option<String>,

    #[arg(long)]
    pub header: Option<PathBuf>
}





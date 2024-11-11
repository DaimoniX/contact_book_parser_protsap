use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: ParseCommands,
}

#[derive(Parser, Debug)]
pub enum ParseCommands {
    #[command(name = "parse", about = "Parse contact book file", long_about = None, arg_required_else_help = true)]
    Parse(ParseFilePath),
}

#[derive(Parser, Debug)]
pub struct ParseFilePath {
    #[clap(short, long)]
    pub input: PathBuf,
}

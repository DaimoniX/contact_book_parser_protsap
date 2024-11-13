use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub help: bool,
    #[command(subcommand)]
    pub cmd: Option<ParseCommands>
}

#[derive(Parser, Debug)]
pub enum ParseCommands {
    #[command(name = "parse", about = "Parse contact book file", long_about = None, arg_required_else_help = true)]
    Parse(ParseFilePath),
}

#[derive(Parser, Debug)]
pub struct ParseFilePath {
    #[clap(short, long, help = "Input file path", long_help = "Full or relative path to the contact book file, example: ./contact_book.toml")]
    pub input: PathBuf,
}

impl Cli {
    pub fn help() -> String {
        String::from("This is a simple contact book parser. It can parse a contact book file in TOML format.\n
        Usage: toml_contact_book_parser [OPTIONS] <SUBCOMMAND>\n
        Options:\n
        -h, --help       Prints help information\n
        -V, --version    Prints version information\n
        Subcommands:\n
        parse -i [file]  Parse contact book file, [file] is a path to the contact book file in TOML format\n
        \n
        See 'toml_contact_book_parser <SUBCOMMAND> --help' for more information on a specific subcommand.\n
        \n
        Example: toml_contact_book_parser parse -i contact_book.toml")
    }
}

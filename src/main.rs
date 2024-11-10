use anyhow::Result;
use contact_book_parser_protsap::parse_contacts;

use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    cmd: ParseCommands,
}

#[derive(Parser, Debug)]
enum ParseCommands {
    #[command(name = "parse", about = "Parse contact book file", long_about = None, arg_required_else_help = true)]
    Parse(ParseFile),
}

#[derive(Parser, Debug)]
struct ParseFile {
    #[clap(short, long)]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Cli::parse();

    match opts.cmd {
        ParseCommands::Parse(parse_file) => {
            let contacts = parse_contacts(
                &std::fs::read_to_string(parse_file.input).expect("Cannot read file"),
            )
            .expect("Cannot parse contacts");
            println!("{:?}", contacts);
        }
    }

    Ok(())
}

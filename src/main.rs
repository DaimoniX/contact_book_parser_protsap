use anyhow::Result;
use clap::Parser;
use toml_contact_book_parser::cli::{Cli, ParseCommands};
use toml_contact_book_parser::parser::parse_from_file;

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.help {
        println!("{}", Cli::help());
        return Ok(());
    }

    if let Some(cmd) = args.cmd {
        match cmd {
            ParseCommands::Parse(file_path) => {
                parse_from_file(file_path.input)?;
            }
        }
    }
    else {
        println!("{}", Cli::help());
    }

    Ok(())
}

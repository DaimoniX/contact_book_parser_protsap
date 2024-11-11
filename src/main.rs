use anyhow::Result;
use clap::Parser;
use toml_contact_book_parser::cli::{Cli, ParseCommands};
use toml_contact_book_parser::parser::parse_from_file;

fn main() -> Result<()> {
    let opts = Cli::parse();

    match opts.cmd {
        ParseCommands::Parse(parse_file) => {
            let contacts = parse_from_file(parse_file.input)?;
            println!("{:?}", contacts);
        }
    }

    Ok(())
}

use anyhow::Result;
use contact_book_parser_protsap::parse_contacts;

fn main() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n";
    let contacts = parse_contacts(str)?;

    println!("{:?}", contacts);

    Ok(())
}

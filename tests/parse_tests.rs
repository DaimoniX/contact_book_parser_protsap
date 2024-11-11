use anyhow::Result;
use toml_contact_book_parser::parser::{contacts_to_string, parse_from_string};

/// Test for parsing contact file (from string)
#[test]
fn test_parse_contact() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n";
    let contacts = parse_from_string(str)?;
    assert_eq!(contacts.len(), 1);

    let contact = &contacts[0];
    assert_eq!(contact.name, "John");
    assert_eq!(contact.surname, "Doe");
    assert_eq!(contact.phones.len(), 2);
    assert_eq!(contact.phones[0], "+380501234567");
    assert_eq!(contact.phones[1], "+380501234568");
    assert_eq!(contact.address, "Some address");
    assert_eq!(contact.birthday, "2000-01-01");

    Ok(())
}

/// Test for converting contacts to string
#[test]
fn test_contacts_to_string() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n";
    let contacts = parse_from_string(str)?;
    let cstr = contacts_to_string(contacts);
    assert_eq!(str, cstr);

    Ok(())
}

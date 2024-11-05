use anyhow::Result;
use contact_book_parser_protsap::parse_contacts;

/// Test for parsing contact file (from string)
#[test]
fn test_parse_contact() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n";
    let contacts = parse_contacts(str)?;
    assert_eq!(contacts.len(), 1);

    let contact = &contacts[0];
    assert_eq!(contact.name, "John");
    assert_eq!(contact.surname, "Doe");
    assert_eq!(contact.phones.len(), 2);
    assert_eq!(contact.phones[0], "+380501234567");
    assert_eq!(contact.phones[1], "+380501234568");
    assert_eq!(contact.address, "Some address");
    assert_eq!(contact.birthday, "2000-01-01");

    println!("{:?}", contacts);

    Ok(())
}

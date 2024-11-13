use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;
use toml_contact_book_parser::parser::{Grammar, Rule};
use pest::Parser;

/// Write tests for name and surname rules
#[test]
fn name_surname_test() -> Result<()> {
    let res = Grammar::parse(Rule::name, "\"John\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"John\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 6);

    let res = Grammar::parse(Rule::surname, "\"Doe\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"Doe\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 5);

    Ok(())
}

/// Test for valid address input
#[test]
fn address_test() -> Result<()> {
    let res = Grammar::parse(Rule::address, "\"Some address\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"Some address\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 14);

    Ok(())
}

/// Another test for valid address input
#[test]
fn address_test2() -> Result<()> {
    let res = Grammar::parse(Rule::address, "\"Some address, 123\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"Some address, 123\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 19);

    Ok(())
}

/// Date test
#[test]
fn date_test() -> Result<()> {
    let res = Grammar::parse(Rule::date, "\"2000-01-01\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"2000-01-01\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 12);

    Ok(())
}

/// Test for invalid date input
#[test]
fn date_fail_test() {
    assert!(Grammar::parse(Rule::date, "\"2000-01-01-\"").is_err());
    assert!(Grammar::parse(Rule::date, "\"2000-01-01 00:00:00\"").is_err());
    assert!(Grammar::parse(Rule::date, "\"2000-01-01 00:00\"").is_err());
    assert!(Grammar::parse(Rule::date, "\"2000-0-01\"").is_err());
    assert!(Grammar::parse(Rule::date, "\"2000\"").is_err());
}

/// Test for valid phone input, simple format
#[test]
fn phone_simple_format_test() -> Result<()> {
    let res = Grammar::parse(Rule::phone, "\"+381233456789\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"+381233456789\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 15);

    let res = Grammar::parse(Rule::phone, "\"381233456789\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"381233456789\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 14);

    let res = Grammar::parse(Rule::phone, "\"+38 123 345 6789\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"+38 123 345 6789\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 18);

    Ok(())
}

/// Test for valid phone input, complex format
#[test]
fn phone_complex_format_test() -> Result<()> {
    let res = Grammar::parse(Rule::phone, "\"+38(123)3456789\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"+38(123)3456789\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 17);

    let res = Grammar::parse(Rule::phone, "\"+38(123)345-6789\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"+38(123)345-6789\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 18);

    let res = Grammar::parse(Rule::phone, "\"+38(123)345-67-89\"")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;

    assert_eq!(res.as_str(), "\"+38(123)345-67-89\"");
    assert_eq!(res.as_span().start(), 0);
    assert_eq!(res.as_span().end(), 19);

    Ok(())
}

/// Test for invalid phone input
#[test]
fn phone_fail_test() {
    assert!(Grammar::parse(Rule::phone, "\"+381233456789").is_err());
    assert!(Grammar::parse(Rule::phone, "").is_err());
    assert!(Grammar::parse(Rule::phone, "\"-38(123)3456789\"").is_err());
    assert!(Grammar::parse(Rule::phone, "\"+38(123)3456789-\"").is_err());
    assert!(Grammar::parse(Rule::phone, "\"+38(123)3\"").is_err());
    assert!(Grammar::parse(Rule::phone, "\"+38123)3456789\"").is_err());
    assert!(Grammar::parse(Rule::phone, "\"+38(1233456789\"").is_err());
    assert!(Grammar::parse(Rule::phone, "\"+38(123)34-56789\"").is_err());
}

/// Test for contact input
#[test]
fn contact_test() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n";
    let contact = Grammar::parse(Rule::contact, str)?.next().unwrap();
    assert!(contact.as_str().contains("John"));
    Ok(())
}

/// Test for parsing file
#[test]
fn file_test() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n";
    let file = Grammar::parse(Rule::file, str)?.next().unwrap();
    assert!(file.as_str().contains("John"));
    assert!(file.as_str().contains("Doe"));
    assert!(file.as_str().contains("Some address"));
    assert!(file.as_str().contains("2000-01-01"));
    Ok(())
}

/// Multiple contacts test
#[test]
fn multiple_contacts_test() -> Result<()> {
    let str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n[contact]\nname = \"Jane\"\nsurname = \"Doe\"\nphones = [\"+380501234567\", \"+380501234568\"]\naddress = \"Some address\"\nbirthday = \"2000-01-01\"\n\n";
    let file = Grammar::parse(Rule::file, str)?.next().unwrap();
    assert!(file.as_str().contains("John"));
    assert!(file.as_str().contains("Jane"));
    Ok(())
}

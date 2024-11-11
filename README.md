# toml_contact_book_parser
## Toml Contact Book Parser

This is a simple parser for contact book, stored in toml format. It can parse a file with contacts and return a list of contacts.
Made for Rust course.

## Table of contents
- [Example of contact book](#example-of-contact-book)
- [Usage example](#usage-example)
- [Grammar](#grammar)

## Example of contact book

```toml
[contact]
name = "John"
surname = "Doe"
phones = ["+380501234567", "+380501234568"]
address = "Some address"
birthday = "2000-01-01"

```

## Usage example

```rust
use toml_contact_book_parser::parse_contacts;

fn main() {
    let raw_contact_str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501..."; // fill with your contact book or load from file
    
    let contacts = parse_contacts(raw_contact_str).unwrap();

    println!("{:?}", contacts);
    // [Contact { name: "John", surname: "Doe", phones: ["+380501234567", "+380501234568"], address: "Some address", birthday: "2000-01-01" }, ...]
}
```

## Grammar

```pest
// Helper rules
equals = _{ " "* ~ "=" ~ " "* }

// Phone helper rules
country_code = _{ ASCII_DIGIT{1,3} }
area_code = _{ ASCII_DIGIT{3} }
exchange_code = _{ ASCII_DIGIT{3} }
subscriber_number = _{ ASCII_DIGIT{4} }
subscriber_number_dashed = _{ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} }
subscriber_number_combo = _{ subscriber_number | subscriber_number_dashed }

// Main rules
name = { "\"" ~ (ASCII_ALPHA)+ ~ "\"" }
surname = { "\"" ~ (ASCII_ALPHA)+ ~ "\"" }
address = { "\"" ~ (ASCII_ALPHANUMERIC | " " | "." | "," | "-")+ ~ "\"" }
phone = { "\"" ~ ("+")? ~ (
    // Without brackets
    (ASCII_DIGIT{11,13}) |
    (country_code ~ " "{0,1} ~ area_code ~ " "{0,1} ~ exchange_code ~ " "{0,1} ~ subscriber_number) |
    (country_code ~ "-" ~ area_code ~ "-" ~ exchange_code ~ "-" ~ subscriber_number) |
    // With brackets
    (country_code ~ " "{0,1} ~ "(" ~ area_code ~ ")" ~ " "{0,1} ~ exchange_code ~ " "{0,1} ~ subscriber_number) |
    (country_code ~ "(" ~ area_code ~ ")" ~ "-"{0,1} ~ exchange_code ~ "-" ~ subscriber_number_combo)
) ~ "\"" }
date = { "\"" ~ ASCII_DIGIT{4} ~ "-" ~ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} ~ "\"" }
phones = { "[" ~ phone ~ ("," ~ " "{0, 1} ~ phone)* ~ "]" }

contact = {
    "[contact]" ~ " "* ~ NEWLINE ~
    "name" ~ equals ~ name ~ NEWLINE ~
    "surname" ~ equals ~ surname ~ NEWLINE ~
    "phones" ~ equals ~ phones ~ NEWLINE ~
    "address" ~ equals ~ address ~ NEWLINE ~
    "birthday" ~ equals ~ date ~ NEWLINE
}
file = { SOI ~ (contact ~ NEWLINE{0,})* ~ EOI }
```

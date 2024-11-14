# toml_contact_book_parser
## Toml Contact Book Parser

This is a simple parser for contact book, stored in toml format. It can parse a file with contacts and return a list of contacts.
Made for Rust course.

## Links
- [Crate](https://crates.io/crates/toml_contact_book_parser)
- [Documentation](https://docs.rs/toml_contact_book_parser)

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

### Toml grammar

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

### Name and surname rules

These rules match any string that contains only ASCII letters and is enclosed in double quotes, e.g. `"John"`, `"Doe"`, `"Carl"`, etc.

```pest
name = { "\"" ~ (ASCII_ALPHA)+ ~ "\"" }
surname = { "\"" ~ (ASCII_ALPHA)+ ~ "\"" }
```

### Address rule

This rule matches any string that contains only ASCII alphanumeric characters, spaces, dots, commas, and dashes, e.g. `"Some address"`, `"Some address, 123"`, `"Some address, 123, apt. 45"`, etc.

```pest
address = { "\"" ~ (ASCII_ALPHANUMERIC | " " | "." | "," | "-")+ ~ "\"" }
```

### Phone rule

This rule matches any phone number in the following formats:
- `+380501234567`
- `+380 50 123 45 67`
- `+380-50-123-45-67`
- `+380 (50) 123 45 67`
- `+380(50)-123-45-67`

```pest
phone = { "\"" ~ ("+")? ~ (
    (ASCII_DIGIT{11,13}) |
    (country_code ~ " "{0,1} ~ area_code ~ " "{0,1} ~ exchange_code ~ " "{0,1} ~ subscriber_number) |
    (country_code ~ "-" ~ area_code ~ "-" ~ exchange_code ~ "-" ~ subscriber_number) |
    (country_code ~ " "{0,1} ~ "(" ~ area_code ~ ")" ~ " "{0,1} ~ exchange_code ~ " "{0,1} ~ subscriber_number) |
    (country_code ~ "(" ~ area_code ~ ")" ~ "-"{0,1} ~ exchange_code ~ "-" ~ subscriber_number_combo)
) ~ "\"" }
```

### Date rule

This rule matches any date in the format `YYYY-MM-DD`, e.g. `"2000-01-01"`, `"2021-12-31"`, etc.

```pest
date = { "\"" ~ ASCII_DIGIT{4} ~ "-" ~ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} ~ "\"" }
```

### Phones rule

This rule matches a list of phone numbers, e.g. `["+380501234567", "+380501234568"]`.

```pest
phones = { "[" ~ phone ~ ("," ~ " "{0, 1} ~ phone)* ~ "]" }
```

### Contact rule

This rule matches a single contact in the following format:

```toml
[contact]
name = "John"
surname = "Doe"
phones = ["+380501234567", "+380501234568"]
address = "Some address"
birthday = "2000-01-01"
```

```pest
contact = {
    "[contact]" ~ " "* ~ NEWLINE ~
    "name" ~ equals ~ name ~ NEWLINE ~
    "surname" ~ equals ~ surname ~ NEWLINE ~
    "phones" ~ equals ~ phones ~ NEWLINE ~
    "address" ~ equals ~ address ~ NEWLINE ~
    "birthday" ~ equals ~ date ~ NEWLINE
}
```

### File rule

This rule matches a file that contains one or more contacts.

```pest
file = { SOI ~ (contact ~ NEWLINE{0,})* ~ EOI }
```


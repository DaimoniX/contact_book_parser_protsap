# Contact book parser

This is a simple parser for contact book. It can parse a file with contacts and return a list of contacts.
Made for Rust course.

## Example of contact book

```
[contact]
name = "John"
surname = "Doe"
phones = ["+380501234567", "+380501234568", ...] // at least one phone
address = "Some address"
birthday = "2000-01-01"

[contact]
...
```

## Usage example

```rust
use contact_book_parser_protsap::parse_contacts;

fn main() {
    let raw_contact_str = "[contact]\nname = \"John\"\nsurname = \"Doe\"\nphones = [\"+380501..."; // fill with your contact book or load from file
    
    let contacts = parse_contacts(raw_contact_str).unwrap();

    println!("{:?}", contacts);
    // [Contact { name: "John", surname: "Doe", phones: ["+380501234567", "+380501234568"], address: "Some address", birthday: "2000-01-01" }, ...]
}
```
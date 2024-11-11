pub mod cli;
pub mod error;

pub mod parser {
    use crate::error::ContactBookParserError;
    use pest::Parser;
    use pest_derive::Parser;
    use std::path::Path;

    #[derive(Parser)]
    #[grammar = "./grammar.pest"]
    pub struct Grammar;

    #[derive(Debug)]
    pub struct Contact {
        pub name: String,
        pub surname: String,
        pub phones: Vec<String>,
        pub address: String,
        pub birthday: String,
    }

    fn remove_quotes(s: &str) -> String {
        s.replace("\"", "")
    }

    /// Parse contacts from a string, returns a vector of contacts
    pub fn parse_from_string(input: &str) -> Result<Vec<Contact>, ContactBookParserError> {
        let contacts = Grammar::parse(Rule::file, input)
            .map_err(|e| ContactBookParserError::ParseError(Box::from(e)))?;
        let mut result: Vec<Contact> = Vec::new();

        for contact in contacts {
            let mut name = String::new();
            let mut surname = String::new();
            let mut phones = Vec::new();
            let mut address = String::new();
            let mut birthday = String::new();

            for record in contact.into_inner() {
                if record.as_rule() == Rule::contact {
                    for contact_record in record.into_inner() {
                        match contact_record.as_rule() {
                            Rule::name => {
                                name = remove_quotes(contact_record.as_str());
                            }
                            Rule::surname => {
                                surname = remove_quotes(contact_record.as_str());
                            }
                            Rule::phones => {
                                for phone in contact_record.into_inner() {
                                    phones.push(remove_quotes(phone.as_str()));
                                }
                            }
                            Rule::address => {
                                address = remove_quotes(contact_record.as_str());
                            }
                            Rule::date => {
                                birthday = remove_quotes(contact_record.as_str());
                            }
                            _ => {}
                        }
                    }
                }
            }

            result.push(Contact {
                name,
                surname,
                phones,
                address,
                birthday,
            });
        }

        Ok(result)
    }

    /// Parse contacts from a file returns a vector of contacts
    pub fn parse_from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<Contact>, ContactBookParserError> {
        let input = std::fs::read_to_string(path).map_err(ContactBookParserError::IOError)?;
        parse_from_string(&input)
    }

    /// Convert a vector of contacts to a valid toml string
    pub fn contacts_to_string(contacts: Vec<Contact>) -> String {
        let mut result = String::new();

        for contact in contacts {
            result.push_str("[contact]\n");
            result.push_str(&format!("name = \"{}\"\n", contact.name));
            result.push_str(&format!("surname = \"{}\"\n", contact.surname));
            result.push_str("phones = [");
            for (i, phone) in contact.phones.iter().enumerate() {
                result.push_str(&format!("\"{}\"", phone));
                if i < contact.phones.len() - 1 {
                    result.push_str(", ");
                }
            }
            result.push_str("]\n");
            result.push_str(&format!("address = \"{}\"\n", contact.address));
            result.push_str(&format!("birthday = \"{}\"\n\n", contact.birthday));
        }

        result
    }
}

use pest::Parser;
use pest_derive::Parser;

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

pub fn parse_contacts(input: &str) -> Result<Vec<Contact>, Box<pest::error::Error<Rule>>> {
    let contacts = Grammar::parse(Rule::file, input)?;
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

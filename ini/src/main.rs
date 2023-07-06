use pest::Parser;
use pest_derive::Parser;

use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

fn main() {
    let unparsed_file =
        fs::read_to_string("config.ini").expect("cannot read file");
    
    let file =
        INIParser::parse(Rule::file, &unparsed_file)
        // unwrap the parse result
        .expect("unsuccessful parse")
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut properties: HashMap<&str, HashMap<&str, &str>> =
        HashMap::new();
    
    // the main loop
    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                // { name }
                let mut inner_rules = line.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str();
            },
            Rule::property => {
                // { name ~ "=" ~ value }
                let mut inner_rules = line.into_inner();

                let name: &str = inner_rules.next().unwrap().as_str();
                let value: &str = inner_rules.next().unwrap().as_str();

                // Insert an empty inner hashmap if the outer hashmap hasn't
                // seen this section before
                let section =
                    properties.entry(current_section_name).or_default();
                section.insert(name, value);
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}

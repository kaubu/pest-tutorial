use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    // let successful_parse =
    //     CSVParser::parse(Rule::field, "-275.13");
    // println!("{:?}", successful_parse);
    
    // let unsuccessful_parse =
    //     CSVParser::parse(Rule::field, "this is not a number!");
    // println!("{:?}", unsuccessful_parse);

    let unparsed_file = fs::read_to_string("numbers.csv")
        .expect("can not read file");

    let file =
        CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap();

    // Iterate over files and fields
    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}

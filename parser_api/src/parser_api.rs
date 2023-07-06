use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser_api.pest"]
pub struct ParserAPIParser;

fn main() {
    let parse_result =
    ParserAPIParser::parse(Rule::sum, "1773 + 1362")
        .unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    // Pairs
    let pair =
        ParserAPIParser::parse(
            Rule::enclosed, "(..6472..) and more text"
        ).unwrap().next().unwrap();
    
    assert_eq!(pair.as_rule(), Rule::enclosed);
    assert_eq!(pair.as_str(), "(..6472..)");

    let inner_rules = pair.into_inner();
    println!("inner rules: {}", inner_rules);

    // Iterating into inner rules
    let pairs =
        ParserAPIParser::parse(Rule::sum, "1773 + 1362")
        .unwrap().next().unwrap()
        .into_inner();

    let numbers = pairs
        .clone()
        .map(|pair| str::parse(pair.as_str()).unwrap())
        .collect::<Vec<i32>>();

    assert_eq!(vec![1773, 1362], numbers);

    for (found, expected) in pairs.zip(
        vec!["1773", "1362"]
    ) {
        assert_eq!(Rule::number, found.as_rule());
        assert_eq!(expected, found.as_str());
    }

    // .next()
    let parse_result =
        ParserAPIParser::parse(Rule::sum, "1773 + 1362")
        .unwrap().next().unwrap();
    let mut inner_rules = parse_result.into_inner();

    let match1 = inner_rules.next().unwrap();
    let match2 = inner_rules.next().unwrap();

    assert_eq!(match1.as_str(), "1773");
    assert_eq!(match2.as_str(), "1362");

    // parse method
    // check whether the parse was successful
    match ParserAPIParser::parse(Rule::enclosed, "(..6472..)") {
        Ok(mut pairs) => {
            let enclosed = pairs.next().unwrap();
        },
        Err(error) => {
            
        },
    }
}
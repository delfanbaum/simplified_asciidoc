use regex::Regex;

#[derive(Debug)]
struct Rule {
    re: Regex,
    to: String,
}

fn get_rules() -> Vec<Rule> {
    vec![
        // header rules
        Rule {
            re: Regex::new(r"={6}\s?([^\n]+)").unwrap(),
            to: "<h6>$1</h6>".to_string(),
        },
        Rule {
            re: Regex::new(r"={5}\s?([^\n]+)").unwrap(),
            to: "<h5>$1</h5>".to_string(),
        },
        Rule {
            re: Regex::new(r"={4}\s?([^\n]+)").unwrap(),
            to: "<h4>$1</h4>".to_string(),
        },
        Rule {
            re: Regex::new(r"={3}\s?([^\n]+)").unwrap(),
            to: "<h3>$1</h3>".to_string(),
        },
        Rule {
            re: Regex::new(r"={2}\s?([^\n]+)").unwrap(),
            to: "<h2>$1</h2>".to_string(),
        },
        Rule {
            re: Regex::new(r"={1}\s?([^\n]+)").unwrap(),
            to: "<h1>$1</h1>".to_string(),
        },
    ]
}

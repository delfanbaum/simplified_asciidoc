use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Inlines {
    Text,
    Bold,
    Italic,
    Code,
    Link,
    Footnote,
}

pub fn get_class_from_role(line: &str) -> Option<String> {
    let re = Regex::new(r#"\[role="(.*?)"\]"#).unwrap();
    let classes = re.captures(line).unwrap();
    match classes.get(1) {
        Some(class) => Some(class.as_str().to_string()),
        None => None,
    }
}

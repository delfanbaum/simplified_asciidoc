use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Inlines {
    Text,
    Bold,
    Italic,
    Code,
    Link(String),
    Footnote,
}

impl Inlines {
    /// includes attributes! ignore for close
    pub fn tag(&self) -> Option<String> {
        match self {
            Inlines::Bold => Some("strong".to_string()),
            Inlines::Italic => Some("em".to_string()),
            Inlines::Code => Some("code".to_string()),
            Inlines::Footnote => Some("span data-type=\"footnote\"".to_string()),
            Inlines::Link(href) => Some(format!("a href=\"{}\"", href)),
            _ => None,
        }
    }

    pub fn open_tag(&self) -> Option<String> {
        match self {
            Inlines::Bold | Inlines::Italic | Inlines::Code => {
                Some(format!("<{}>", self.tag().unwrap()))
            }
            Inlines::Footnote => Some("<span data-type=\"footnote\">".to_string()),
            Inlines::Link(href) => Some(format!("<a href=\"{}\">", href)),
            _ => None,
        }
    }
    pub fn close_tag(&self) -> Option<String> {
        match self {
            Inlines::Bold | Inlines::Italic | Inlines::Code => {
                Some(format!("<{}>", self.tag().unwrap()))
            }
            Inlines::Footnote => Some("</span>".to_string()),
            Inlines::Link(_) => Some("</a>".to_string()),
            _ => None,
        }
    }

    pub fn convert_complete_tags(line: String) -> (String, Self) {
        // loop through the regexes
        // see "strum" crate for looping through enums; then loop through and write a function
        // that returns the regex replace per enum, and then -- what. well, do it!
        todo!()

    }
}

pub fn get_class_from_role(line: &str) -> Option<String> {
    let re = Regex::new(r#"\[role="(.*?)"\]"#).unwrap();
    let classes = re.captures(line).unwrap();
    classes.get(1).map(|class| class.as_str().to_string())
}

use crate::blocks::*;
use crate::inlines::{get_class_from_role, Inlines};

#[derive(Debug)]
pub struct Parser {
    current_parent_block: Option<ParentBlock>,
    current_block: Option<Blocks>,
    current_inline: Option<Vec<Inlines>>,
    current_class: Option<String>,
    in_tag: bool,
    /// The HTML "output"
    parsed_markup: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current_parent_block: None,
            current_block: None,
            current_inline: None,
            current_class: None,
            in_tag: false,
            parsed_markup: String::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        let splits: Vec<&str> = line.split_whitespace().collect();

        // Checks to see if the line is just block syntax markup
        if !self.is_block_syntax(&splits) {
            // Checks to see if the line begins with some inline block markup
            if !self.is_inline_block_marker(&splits) {
                match line.chars().next().unwrap() {
                    '[' => match &splits[0][..5] {
                        // to do: attributions! attrs generally, actually
                        "[quot" => self.current_parent_block = Some(ParentBlock::Quote),
                        "[vers" => self.current_parent_block = Some(ParentBlock::Verse),
                        "[role" => self.current_class = get_class_from_role(line),
                        _ => eprintln!("Unhandled block tag: {}", line),
                    },
                    _ => {
                        // if not in a paragraph already, make a paragraph
                        if *self.current_parent_block.as_ref().unwrap() != ParentBlock::Paragraph {
                            self.current_parent_block = Some(ParentBlock::Paragraph);
                        }
                        self.current_block = Some(Blocks::Paragraph)
                    }
                }
            }
        }
        if self.current_block.is_some() {
            self.add_line(line)
        }
    }

    fn is_block_syntax(&mut self, splits: &[&str]) -> bool {
        if splits.is_empty() {
            // basically, clear
            // i.e., a "\n" string
            self.current_parent_block = None;
            self.current_block = None;
            self.current_inline = None;
            self.current_class = None;
            // we may also need a close inline tags
            self.close_tags();
            return true;
        } else if splits.len() == 1 {
            match splits[0] {
                "'''" => {
                    self.current_block = Some(Blocks::Break);
                    self.current_class = Some("section_break".to_string())
                }
                ">>>" => {
                    self.current_block = Some(Blocks::Break);
                    self.current_class = Some("page_break".to_string())
                }
                "****" => self.current_parent_block = Some(ParentBlock::Aside),
                "----" => self.current_parent_block = Some(ParentBlock::Pre),
                // We probably don't need to handle this, since it should have [verse] or
                // [quote] before it
                //"____" => self.current_block = Some(Blocks::Code),
                _ => return false, // this allows for things like [verse], [role], etc.
            };
            return true;
        }
        false
    }

    fn is_inline_block_marker(&mut self, splits: &[&str]) -> bool {
        match splits[0].chars().next().unwrap() {
            '=' => {
                // use the length of the first split to get heading level
                self.current_block = Some(Blocks::Heading(splits[0].len()));
                true
            }
            '*' => {
                if self.current_block.is_none()
                    || *self.current_parent_block.as_ref().unwrap() == ParentBlock::UnorderedList
                    || *self.current_block.as_ref().unwrap() != Blocks::Paragraph
                {
                    self.current_block = Some(Blocks::ListItem);
                    self.current_parent_block = Some(ParentBlock::UnorderedList)
                };
                true
            }
            '.' => {
                if self.current_block.is_none()
                    || *self.current_parent_block.as_ref().unwrap() == ParentBlock::OrderedList
                    || *self.current_block.as_ref().unwrap() != Blocks::Paragraph
                {
                    self.current_block = Some(Blocks::ListItem);
                    self.current_parent_block = Some(ParentBlock::OrderedList)
                };
                true
            }
            _ => false,
        }
    }

    fn open_block_tags(&mut self) {
        let mut tag = String::new();
        if self.current_parent_block.is_some() {
            tag.push_str(&format!("<{}>", self.current_block.unwrap().tag()))
        }
        if self.current_block.is_some() {
            tag.push_str(&format!("<{}>", self.current_block.unwrap().tag()))
        }
        if self.current_class.is_some() {
            tag = tag.replacen(
                '>',
                &format!(" class=\"{}\">", self.current_class.as_ref().unwrap()),
                1,
            );
        }
        self.parsed_markup.push_str(&tag);
    }

    fn close_tags(&mut self) {
        let mut tag = String::new();
        match &self.current_inline {
            Some(inlines) => {
                for inline in inlines.iter() {
                    if inline.tag().is_some() {
                        tag.push_str(&format!(
                            "</{}>",
                            inline.tag().unwrap().split_whitespace().next().unwrap()
                        ))
                    }
                }
            }
            None => (),
        }
        if self.current_parent_block.is_some() {
            tag.push_str(&format!("</{}>", self.current_block.unwrap().tag()))
        }
        if self.current_block.is_some() {
            tag.push_str(&format!("</{}>", self.current_block.unwrap().tag()))
        }
        self.parsed_markup.push_str(&tag);
    }

    fn add_line(&mut self, line: &str) {
        let mut html_fragment = String::new();
        if !self.in_tag {
            self.open_block_tags();
        }
        let line = match self.current_block.unwrap() {
            Blocks::Heading(_) | Blocks::ListItem => {
                line.to_string().split_whitespace().collect::<Vec<&str>>()[1..].join(" ")
            }
            _ => line.to_string(),
        };
        let parsed_line = self.parse_inline(line);
        html_fragment.push_str(&parsed_line);

        // add to parser
        self.parsed_markup.push_str(&html_fragment);
    }

    fn parse_inline(&mut self, line: String) -> String {
        // really this is "TODO" but we want the other test to pass
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let mut p = Parser::new();
        for heading_level in vec![1, 2, 3, 4, 5, 6] {
            let line = format!("{} This is a heading", "=".repeat(heading_level));
            p.parse_line(&line);
            assert_eq!(p.current_block.unwrap(), Blocks::Heading(heading_level));
        }
    }

    #[test]
    fn test_parse_newline() {
        let mut p = Parser::new();
        let line = "\n";
        p.parse_line(line);
        assert!(p.current_block.is_none());
    }

    #[test]
    fn test_parse_lists() {
        let mut p = Parser::new();
        let line = ". My Item";
        p.parse_line(line);
        assert_eq!(p.current_block.unwrap(), Blocks::ListItem);
        assert_eq!(p.current_parent_block.unwrap(), ParentBlock::OrderedList);

        let line = "* My Item";
        p.parse_line(line);
        assert_eq!(p.current_block.unwrap(), Blocks::ListItem);
        assert_eq!(p.current_parent_block.unwrap(), ParentBlock::UnorderedList);
    }

    #[test]
    fn handle_bracketed_lines() {
        let mut p = Parser::new();
        let line = "[role=\"something\"]";
        p.parse_line(line);
        assert_eq!(*p.current_class.as_ref().unwrap(), "something");

        let line = "[quote]";
        p.parse_line(line);
        assert_eq!(p.current_parent_block.unwrap(), ParentBlock::Quote);

        let line = "[verse]";
        p.parse_line(line);
        assert_eq!(p.current_parent_block.unwrap(), ParentBlock::Verse);
    }

    #[test]
    fn parse_inline_text() {
        let mut p = Parser::new();
        let line = "This is just a line of text. We have an incomplete".to_string();
        assert_eq!(
            "This is just a line of text. We have an incomplete".to_string(),
            p.parse_inline(line)
        )
    }
    #[test]
    fn parse_inline_complete_tags() {
        let mut p = Parser::new();
        let line = "We have an _italic_ part in here".to_string();
        assert_eq!(
            "We have an <em>italic</em> part in here".to_string(),
            p.parse_inline(line)
        );

        let line = "We have a *bold* part in here".to_string();
        assert_eq!(
            "We have a <strong>bold</strong> part in here".to_string(),
            p.parse_inline(line)
        );

        let line = "We have a `code` part in here".to_string();
        assert_eq!(
            "We have a <code>code</code> part in here".to_string(),
            p.parse_inline(line)
        );

        let line = "We have a footnote.footnote[Some text.]".to_string();
        assert_eq!(
            "We have a footnote.<span data-type=\"footnote\">Some text.</span>".to_string(),
            p.parse_inline(line)
        );
    }
}

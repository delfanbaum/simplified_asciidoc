use crate::blocks::*;
use crate::inlines::{get_class_from_role, parse_inline, Inlines};

#[derive(Debug)]
pub struct Parser {
    current_parent_block: Option<ParentBlock>,
    current_block: Option<Blocks>,
    current_inline: Option<Vec<Inlines>>,
    current_class: Option<String>,
    parsed_markup: String,
    in_tag: bool,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            current_parent_block: None,
            current_block: None,
            current_inline: None,
            current_class: None,
            parsed_markup: String::new(),
            in_tag: false,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let splits: Vec<&str> = line.split_whitespace().collect();

        // Checks to see if the line is just block syntax markup
        if !self.is_block_syntax(&splits) {
            // Checks to see if the line begins with some inline block markup
            if !self.is_inline_block_marker(&splits) {
                match line.chars().next().unwrap() {
                    '[' => match &splits[0][..5] {
                        "[quot" => self.current_parent_block = Some(ParentBlock::Quote),
                        "[vers" => self.current_parent_block = Some(ParentBlock::Verse),
                        "[role" => self.current_class = get_class_from_role(&line),
                        _ => println!("{}", splits[0][..4].to_string()),
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
        self.add_line(splits)
    }

    fn is_block_syntax(&mut self, splits: &Vec<&str>) -> bool {
        if splits.is_empty() {
            // basically, clear
            // i.e., a "\n" string
            self.current_parent_block = None;
            self.current_block = None;
            self.current_inline = None;
            self.current_class = None;
            // TODO call "close tag" function
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
                "****" => self.current_block = Some(Blocks::Aside),
                "----" => self.current_block = Some(Blocks::Code),
                // We probably don't need to handle this, since it should have [verse] or
                // [quote] before it
                //"____" => self.current_block = Some(Blocks::Code),
                _ => return false, // this allows for things like [verse], [role], etc.
            };
            return true;
        }
        false
    }

    fn is_inline_block_marker(&mut self, splits: &Vec<&str>) -> bool {
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

    fn add_open_block_tags(&mut self) -> String {
        let mut tag = String::new();
        if self.current_parent_block.is_some() {
            tag.push_str(&format!("<{}>", self.current_block.unwrap().tag()))
        }
        if self.current_block.is_some() {
            tag.push_str(&format!("<{}>", self.current_block.unwrap().tag()))
        }
        if self.current_class.is_some() {
            tag = tag.replacen(
                ">",
                &format!(" class=\"{}\">", self.current_class.as_ref().unwrap()),
                1,
            );
        }
        tag
    }

    fn add_line(&mut self, splits: Vec<&str>) {
        let mut html_fragment = String::new();
        if !self.in_tag {
            html_fragment.push_str(&self.add_open_block_tags())
        }
        let line = match self.current_block.unwrap() {
            Blocks::Heading(_) | Blocks::ListItem => splits[1..].join(" "),
            _ => splits.join(" "),
        };
        let (parsed_line, inline) = parse_inline(line, self.current_inline);
        html_fragment.push_str(&parsed_line);

        // add to parser
        self.parsed_markup.push_str(&html_fragment);
        self.current_inline = inline;
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
        assert_eq!(p.current_block.unwrap(), Blocks::Quote);

        let line = "[verse]";
        p.parse_line(line);
        assert_eq!(p.current_block.unwrap(), Blocks::Verse);
    }
}

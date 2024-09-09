#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParentBlock {
    Section,
    OpenBlock,
    Aside,
    Paragraph,
    OrderedList,
    UnorderedList,
    DefinitionList,
    Quote,
    Verse,
    Pre,
}

impl ParentBlock {
    pub fn tag(&self) -> String {
        match self {
            ParentBlock::Section => "section".to_string(),
            ParentBlock::OpenBlock => "div".to_string(),
            ParentBlock::Paragraph => "p".to_string(),
            ParentBlock::OrderedList => "ol".to_string(),
            ParentBlock::UnorderedList => "ul".to_string(),
            ParentBlock::DefinitionList => "dl".to_string(),
            ParentBlock::Aside => "aside".to_string(),
            ParentBlock::Quote => "blockquote".to_string(),
            ParentBlock::Verse => "pre".to_string(),
            ParentBlock::Pre => "pre".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Blocks {
    Heading(usize),
    Paragraph,
    ListItem,
    DefinitionTerm,
    DefinitionDesc,
    Break,
}

impl Blocks {
    pub fn tag(&self) -> String {
        match self {
            Blocks::Heading(usize) => format!("h{}", usize),
            Blocks::Paragraph => "p".to_string(),
            Blocks::ListItem => "li".to_string(),
            Blocks::DefinitionTerm => "dt".to_string(),
            Blocks::DefinitionDesc => "dd".to_string(),
            Blocks::Break => "div".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Breaks {
    Section,
    Page,
}

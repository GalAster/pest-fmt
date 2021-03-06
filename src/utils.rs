use crate::Settings;
use pest::Span;
use std::fmt::{Debug, Error, Formatter};
pub use textwrap::indent;

impl Default for Settings {
    fn default() -> Self {
        Settings {
            // tab = 4 space
            pest_indent: 4,
            pest_set_alignment: true,
            pest_blank_lines: None,
            pest_choice_first: true,
            pest_choice_hanging: false,
            pest_set_space: 1,
            pest_choice_space: 0,
            pest_braces_space: 0,
            pest_sequence_space: 1,
            pest_parentheses_space: 0,
        }
    }
}

impl Settings {
    pub fn style(s: &str) -> Settings {
        match s {
            _ => Settings::default(),
        }
    }
}

pub fn is_one_line(span: Span) -> bool {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return s == e;
}

pub fn get_lines(span: Span) -> (usize, usize) {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return (s, e);
}

#[derive(Clone)]
pub struct GrammarRule {
    pub is_comment: bool,
    pub identifier: String,
    pub modifier: String,
    pub code: String,
    pub lines: (usize, usize),
}

impl GrammarRule {
    pub fn comment(c: &str) -> Self {
        GrammarRule {
            /// is_comment_or_blank_line
            is_comment: true,
            identifier: "".to_string(),
            modifier: "".to_string(),
            code: c.to_string(),
            lines: (0, 0),
        }
    }
    pub fn to_string(&self, indent: usize) -> String {
        if self.is_comment {
            return self.code.clone();
        }
        let mut code = self.identifier.clone();
        while code.chars().count() < indent {
            code.push_str(" ")
        }
        code.push_str(" = ");
        code.push_str(&self.modifier);
        code.push_str(&self.code);
        return code;
    }
}

impl Debug for GrammarRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {:?}", self.identifier, self.lines)
    }
}

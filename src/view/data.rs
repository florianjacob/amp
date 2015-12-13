use rustbox::Color;
use scribe::buffer::{Position, Range, Token};

pub struct Data {
    pub buffer_id: usize,
    pub tokens: Option<Vec<Token>>,
    pub cursor: Option<Position>,
    pub highlight: Option<Range>,
    pub line_count: usize,
    pub status_line: StatusLine
}

pub struct StatusLine {
    pub content: String,
    pub color: Option<Color>
}

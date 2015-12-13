extern crate rustbox;
extern crate scribe;

pub mod modes;
pub mod buffer;
mod scrollable_region;
pub mod terminal;
mod data;
mod color;

// Published API
pub use self::data::{Data, StatusLine};

use helpers;
use self::terminal::Terminal;
use view::buffer::BufferView;

use std::collections::HashMap;
use std::ops::Deref;

use pad::PadStr;
use rustbox::Color;
use scribe::buffer::{Buffer, LineRange, Position, Token};

const LINE_LENGTH_GUIDE_OFFSET: usize = 80;

pub enum Theme {
    Dark,
    Light,
}

pub struct View {
    pub theme: Theme,
    terminal: Terminal,
    pub buffer_view: BufferView,
    scroll_offsets: HashMap<usize, usize>,
}

impl Deref for View {
    type Target = Terminal;

    fn deref(&self) -> &Terminal {
        &self.terminal
    }
}

impl View {
    pub fn new() -> View {
        let terminal = Terminal::new();
        let height = terminal.height()-1;

        View{
            theme: Theme::Dark,
            terminal: terminal,
            buffer_view: BufferView::new(height),
            scroll_offsets: HashMap::new(),
        }
    }

    pub fn draw_tokens(&self, data: &Data) {
        let mut line = 0;
        let default_offset = 0;

        // Get the visible set of tokens.
        let offset: usize = *self.scroll_offsets.
            get(&data.buffer_id).
            unwrap_or(&default_offset);
        let visible_range = LineRange::new(
            offset,
            self.terminal.height() + offset,
        );
        let tokens = match data.tokens {
            Some(ref tokens) => visible_tokens(tokens, visible_range),
            None => return,
        };

        // Determine the gutter size based on the number of lines.
        let line_number_width = data.line_count.to_string().len() + 1;
        let gutter_width = line_number_width + 2;

        // Set the terminal cursor, considering leading line numbers.
        match data.cursor {
            Some(position) => {
                self.terminal.set_cursor(
                    (position.offset + gutter_width) as isize,
                    position.line as isize
                );
            },
            None => (),
        }

        // Draw the first line number.
        // Others will be drawn following newline characters.
        let mut offset = self.draw_line_number(
            0,
            data,
            line_number_width
        );

        for token in tokens.iter() {
            let token_color = color::map(&token.category);

            for character in token.lexeme.chars() {
                let current_position = Position{
                    line: line,
                    offset: offset - gutter_width
                };

                let (style, color) = match data.highlight {
                    Some(ref highlight_range) => {
                        if highlight_range.includes(&current_position) {
                            (rustbox::RB_REVERSE, Color::Default)
                        } else {
                            (rustbox::RB_NORMAL, token_color)
                        }
                    },
                    None => (rustbox::RB_NORMAL, token_color),
                };

                let background_color = match data.cursor {
                    Some(cursor) => {
                        if line == cursor.line {
                            self.alt_background_color()
                        } else {
                            Color::Default
                        }
                    },
                    None => Color::Default,
                };

                if character == '\n' {
                    // Print the rest of the line highlight.
                    match data.cursor {
                        Some(cursor) => {
                            if line == cursor.line {
                                for offset in offset..self.terminal.width() {
                                    self.terminal.print_char(
                                        offset,
                                        line,
                                        style,
                                        Color::Default,
                                        self.alt_background_color(),
                                        ' '
                                    );
                                }
                            }
                        }
                        None => (),
                    }

                    // Print the length guide for this line.
                    if offset <= LINE_LENGTH_GUIDE_OFFSET {
                        self.terminal.print_char(
                            LINE_LENGTH_GUIDE_OFFSET,
                            line,
                            rustbox::RB_NORMAL,
                            Color::Default,
                            self.alt_background_color(),
                            ' '
                        );
                    }

                    // Advance to the next line.
                    line += 1;

                    // Draw leading line number for the new line.
                    offset = self.draw_line_number(
                        line,
                        data,
                        line_number_width
                    );
                } else {
                    self.terminal.print_char(
                        offset,
                        line,
                        style,
                        color,
                        background_color,
                        character
                    );

                    offset += 1;
                }
            }
        }

        // Print the rest of the line highlight.
        match data.cursor {
            Some(cursor) => {
                if line == cursor.line {
                    for offset in offset..self.terminal.width() {
                        self.terminal.print_char(
                            offset,
                            line,
                            rustbox::RB_NORMAL,
                            Color::Default,
                            self.alt_background_color(),
                            ' '
                        );
                    }
                }
            },
            None => (),
        }
    }

    pub fn draw_status_line(&self, content: &str, color: Option<Color>) {
        let line = self.terminal.height()-1;
        self.terminal.print(
            0,
            line,
            rustbox::RB_BOLD,
            Color::Default,
            color.unwrap_or(self.alt_background_color()),
            &content.pad_to_width(self.terminal.width())
        );
    }

    fn draw_line_number(&self, line: usize, data: &Data, width: usize) -> usize {
        let mut offset = 0;

        // Line numbers are zero-based and relative;
        // get non-zero-based absolute version.
        let absolute_line = line + data.scrolling_offset + 1;

        // Get left-padded string-based line number.
        let line_number = format!(
            "{:>width$}  ",
            absolute_line,
            width=width
        );

        // Print numbers.
        for number in line_number.chars() {
            // Numbers (and their leading spaces) have background
            // color, but the right-hand side gutter gap does not.
            let background_color = match data.cursor {
                Some(cursor) => {
                    if offset > width && line != cursor.line {
                        Color::Default
                    } else {
                        self.alt_background_color()
                    }
                },
                None => {
                    if offset > width {
                        Color::Default
                    } else {
                        self.alt_background_color()
                    }
                },
            };

            // Current line number is emboldened.
            let weight = match data.cursor {
                Some(cursor) => {
                    if line == cursor.line {
                        rustbox::RB_BOLD
                    } else {
                        rustbox::RB_NORMAL
                    }
                },
                None => rustbox::RB_NORMAL
            };

            self.terminal.print_char(
                offset,
                line,
                weight,
                Color::Default,
                background_color,
                number
            );

            offset += 1;
        }

        offset
    }

    pub fn alt_background_color(&self) -> Color {
        match self.theme {
            Theme::Dark  => Color::Black,
            Theme::Light => Color::White,
        }
    }

    pub fn scroll_up(&mut self, buffer: &Buffer, amount: usize) {
        let key = helpers::buffer_id(buffer);

        match self.scroll_offsets.get_mut(&key) {
            Some(offset) => *offset = offset.checked_sub(amount).unwrap_or(0),
            None => (),
        }
    }

    pub fn scroll_down(&mut self, buffer: &Buffer, amount: usize) {
        let key = helpers::buffer_id(buffer);

        if self.scroll_offsets.contains_key(&key) {
            match self.scroll_offsets.get_mut(&key) {
                Some(offset) => *offset = offset.checked_add(amount).unwrap_or(0),
                None => (),
            }
        } else {
            self.scroll_offsets.insert(key, amount);
        }
    }
}

fn visible_tokens(tokens: &Vec<Token>, visible_range: LineRange) -> Vec<Token> {
    let mut visible_tokens = Vec::new();
    let mut line = 0;

    for token in tokens {
        let mut current_lexeme = String::new();

        for character in token.lexeme.chars() {
            // Use characters in the visible range.
            if visible_range.includes(line) {
                current_lexeme.push(character);
            }

            // Handle newline characters.
            if character == '\n' {
                line += 1;
            }
        }

        // Add visible lexemes to the token set.
        if !current_lexeme.is_empty() {
            visible_tokens.push(Token{
                lexeme: current_lexeme,
                category: token.category.clone()
            })
        }
    }

    visible_tokens
}

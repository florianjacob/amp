extern crate scribe;
extern crate luthor;

use commands;
use helpers::token::{Direction, adjacent_token_position};
use models::application::Application;
use scribe::buffer::{Position};
use super::{application, buffer};

pub fn move_up(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_up(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_down(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_down(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_left(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_left(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_right(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_right(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_start_of_line(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_to_start_of_line(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_first_line(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_to_first_line(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_last_line(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_to_last_line(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_first_word_of_line(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => {
            // Get the current line.
            match buffer.data().lines().nth(buffer.cursor.line) {
                Some(line) => {
                    // Find the offset of the first non-whitespace character.
                    for (offset, character) in line.chars().enumerate() {
                        if !character.is_whitespace() {
                            // Move the cursor to this position.
                            let new_cursor_position = scribe::buffer::Position {
                                line: buffer.cursor.line,
                                offset: offset,
                            };
                            buffer.cursor.move_to(new_cursor_position);

                            // Stop enumerating; we've done the job.
                            return;
                        }
                    }
                }
                None => (),
            }
        }
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_end_of_line(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => buffer.cursor.move_to_end_of_line(),
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn insert_at_end_of_line(app: &mut Application) {
    move_to_end_of_line(app);
    application::switch_to_insert_mode(app);
    commands::view::scroll_to_cursor(app);
}

pub fn insert_at_first_word_of_line(app: &mut Application) {
    move_to_first_word_of_line(app);
    application::switch_to_insert_mode(app);
    commands::view::scroll_to_cursor(app);
}

pub fn insert_with_newline(app: &mut Application) {
    move_to_end_of_line(app);
    buffer::start_command_group(app);
    buffer::insert_newline(app);
    application::switch_to_insert_mode(app);
    commands::view::scroll_to_cursor(app);
}

pub fn insert_with_newline_above(app: &mut Application) {
    move_to_start_of_line(app);
    buffer::start_command_group(app);
    buffer::insert_newline(app);
    commands::cursor::move_up(app);
    application::switch_to_insert_mode(app);
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_start_of_previous_token(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => {
            match adjacent_token_position(buffer, false, Direction::Backward) {
                Some(position) => {
                    buffer.cursor.move_to(position);
                }
                None => (),
            };
        }
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_start_of_next_token(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => {
            match adjacent_token_position(buffer, false, Direction::Forward) {
                Some(position) => {
                    buffer.cursor.move_to(position);
                }
                None => (),
            };
        }
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn move_to_end_of_current_token(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => {
            match adjacent_token_position(buffer, true, Direction::Forward) {
                Some(position) => {
                    buffer.cursor.move_to(Position {
                        line: position.line,
                        offset: position.offset,
                    });
                }
                None => (),
            };
        }
        None => (),
    }
    commands::view::scroll_to_cursor(app);
}

pub fn append_to_current_token(app: &mut Application) {
    match app.workspace.current_buffer() {
        Some(buffer) => {
            match adjacent_token_position(buffer, true, Direction::Forward) {
                Some(position) => {
                    buffer.cursor.move_to(position);
                }
                None => (),
            };
        }
        None => (),
    }
    application::switch_to_insert_mode(app);
}

#[cfg(test)]
mod tests {
    extern crate scribe;

    use scribe::Buffer;
    use scribe::buffer::Position;
    use models::application::Application;

    #[test]
    fn move_to_first_word_of_line_works() {
        // Set up the application.
        let mut app = set_up_application("    amp");

        // Move to the end of the line.
        let position = scribe::buffer::Position {
            line: 0,
            offset: 7,
        };
        app.workspace.current_buffer().unwrap().cursor.move_to(position);

        // Call the command.
        super::move_to_first_word_of_line(&mut app);

        // Ensure that the cursor is moved to the start of the first word.
        assert_eq!(*app.workspace.current_buffer().unwrap().cursor,
                   Position {
                       line: 0,
                       offset: 4,
                   });
    }

    #[test]
    fn move_to_start_of_previous_token_works() {
        // Set up the application.
        let mut app = set_up_application("\namp editor");

        // Move past the first non-whitespace token.
        app.workspace.current_buffer().unwrap().cursor.move_to(Position {
            line: 1,
            offset: 2,
        });

        // Call the command.
        super::move_to_start_of_previous_token(&mut app);

        // Ensure that the cursor is moved to the start of the previous word.
        assert_eq!(*app.workspace.current_buffer().unwrap().cursor,
                   Position {
                       line: 1,
                       offset: 0,
                   });
    }

    #[test]
    fn move_to_start_of_previous_token_skips_whitespace() {
        // Set up the application.
        let mut app = set_up_application("\namp editor");

        // Move to the start of the second non-whitespace word.
        app.workspace.current_buffer().unwrap().cursor.move_to(Position {
            line: 1,
            offset: 4,
        });

        // Call the command.
        super::move_to_start_of_previous_token(&mut app);

        // Ensure that the cursor is moved to the start of the previous word.
        assert_eq!(*app.workspace.current_buffer().unwrap().cursor,
                   Position {
                       line: 1,
                       offset: 0,
                   });
    }

    #[test]
    fn move_to_start_of_next_token_works() {
        // Set up the application.
        let mut app = set_up_application("\namp editor");

        // Move to the start of the first non-whitespace word.
        app.workspace.current_buffer().unwrap().cursor.move_to(Position {
            line: 1,
            offset: 0,
        });

        // Call the command.
        super::move_to_start_of_next_token(&mut app);

        // Ensure that the cursor is moved to the start of the next word.
        assert_eq!(*app.workspace.current_buffer().unwrap().cursor,
                   Position {
                       line: 1,
                       offset: 4,
                   });
    }

    #[test]
    fn move_to_end_of_current_token_works() {
        // Set up the application and run the command.
        let mut app = set_up_application("\namp editor");

        // Move to the start of the first non-whitespace word.
        app.workspace.current_buffer().unwrap().cursor.move_to(Position {
            line: 1,
            offset: 0,
        });

        // Call the command.
        super::move_to_end_of_current_token(&mut app);

        // Ensure that the cursor is moved to the end of the current word.
        assert_eq!(*app.workspace.current_buffer().unwrap().cursor,
                   Position {
                       line: 1,
                       offset: 3,
                   });
    }

    #[test]
    fn append_to_current_token_works() {
        // Set up the application.
        let mut app = set_up_application("\namp editor");

        // Move to the start of the first non-whitespace word.
        app.workspace.current_buffer().unwrap().cursor.move_to(Position {
            line: 1,
            offset: 0,
        });

        // Call the command.
        super::append_to_current_token(&mut app);

        // Ensure that the cursor is moved to the end of the current word.
        assert_eq!(*app.workspace.current_buffer().unwrap().cursor,
                   Position {
                       line: 1,
                       offset: 3,
                   });

        // Ensure that we're in insert mode.
        assert!(match app.mode {
            ::models::application::Mode::Insert(_) => true,
            _ => false,
        });
    }

    fn set_up_application(content: &str) -> Application {
        let mut app = ::models::application::new();
        let mut buffer = Buffer::new();

        // Insert data with indentation and move to the end of the line.
        buffer.insert(content);

        // Now that we've set up the buffer, add it to the application.
        app.workspace.add_buffer(buffer);

        app
    }
}

extern crate scribe;

pub mod movement_lexer;

use scribe::buffer::{Buffer, LineRange, Position, Range};

/// Translates a line range to a regular range, including its last line.
/// Handles ranges including and end line without trailing newline character.
pub fn inclusive_range(line_range: &LineRange, buffer: &mut Buffer) -> Range {
    let data = buffer.data();
    let next_line = line_range.end() + 1;
    let line_count = data.chars().filter(|&c| c == '\n').count() + 1;
    let end_position =
        if line_count > next_line {
            // There's a line below the end of the range, so just use that
            // to capture the last line and its trailing newline character.
            Position{ line: next_line, offset: 0 }
        } else {
            // There isn't a line below the end of the range, so try to get
            // the last line's length and use that as the ending offset.
            match data.lines().nth(line_range.end()) {
                Some(line_content) => {
                    // Found the last line's content; use it.
                    Position{ line: line_range.end(), offset: line_content.len() }
                },
                // Couldn't find any content for the last line; use a zero offset.
                None => Position{ line: line_range.end(), offset: 0 }
            }
        };

    Range::new(
        Position{ line: line_range.start(), offset: 0 },
        end_position
    )
}

// Converts the buffer's path into an owned String.
// Used as a key for tracking scrollable region offsets.
pub fn buffer_id(buffer: &Buffer) -> usize {
    (buffer as *const Buffer) as usize
}

#[cfg(test)]
mod tests {
    extern crate scribe;

    use scribe::Buffer;
    use scribe::buffer::{LineRange, Position, Range};

    #[test]
    fn inclusive_range_works_correctly_without_trailing_newline() {
        let mut buffer = Buffer::new();
        buffer.insert("amp\neditor");
        let range = LineRange::new(1, 1);

        assert_eq!(
            super::inclusive_range(&range, &mut buffer),
            Range::new(Position{ line: 1, offset: 0 }, Position{ line: 1, offset: 6 })
        );
    }

    #[test]
    fn inclusive_range_works_correctly_with_trailing_newline() {
        let mut buffer = Buffer::new();
        buffer.insert("amp\neditor\n");
        let range = LineRange::new(1, 1);

        assert_eq!(
            super::inclusive_range(&range, &mut buffer),
            Range::new(Position{ line: 1, offset: 0 }, Position{ line: 2, offset: 0 })
        );
    }
}

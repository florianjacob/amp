use helpers;
use models::application::Mode;
use view::{Data, StatusLine};
use scribe::buffer::{Buffer, Range};
use rustbox::Color;

pub fn data(buffer: Option<&mut Buffer>, mode: &mut Mode) -> Data {
    match buffer {
        Some(buf) => {
            // Build status line data.
            let content = match buf.path {
                Some(ref path) => path.to_string_lossy().into_owned(),
                None => String::new(),
            };
            let color = match mode {
                &mut Mode::Insert(_) => Some(Color::Green),
                _ => None,
            };

            // If we're in select mode, get the selected range.
            let highlight = match mode {
                &mut Mode::Select(ref select_mode) => {
                    Some(Range::new(
                        select_mode.anchor,
                        *buf.cursor
                    ))
                },
                &mut Mode::SelectLine(ref mode) => {
                    let range = mode.to_range(&*buf.cursor);

                    Some(Range::new(
                        range.start(),
                        range.end()
                    ))
                },
                _ => None,
            };

            Data{
                buffer_id: helpers::buffer_id(buf),
                tokens: Some(buf.tokens()),
                cursor: Some(*buf.cursor.clone()),
                highlight: highlight,
                line_count: buf.data().lines().count(),
                status_line: StatusLine{
                    content: content,
                    color: color
                }
            }
        }
        None => Data{
            buffer_id: 0,
            tokens: None,
            cursor: None,
            highlight: None,
            line_count: 0,
            status_line: StatusLine{
                content: "".to_string(),
                color: None,
            }
        },
    }
}

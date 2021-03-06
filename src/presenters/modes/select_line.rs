extern crate rustbox;
extern crate scribe;

use models::application::modes::select_line::SelectLineMode;
use scribe::buffer::{Buffer, Position};
use presenters::{buffer_status_line_data, line_count, relative_range, visible_tokens};
use view::{BufferData, StatusLineData, View};
use view::scrollable_region::Visibility;
use rustbox::Color;

pub fn display(buffer: Option<&mut Buffer>, mode: &SelectLineMode, view: &mut View) {
    // Wipe the slate clean.
    view.clear();

    if let Some(buf) = buffer {
        let line_offset = view.visible_region(buf).line_offset();
        let visible_range = view.visible_region(buf).visible_range();

        // Get the buffer's tokens and reduce them to the visible set.
        let visible_tokens = visible_tokens(&buf.tokens(), visible_range);

        // The buffer tracks its cursor absolutely, but the view must display it
        // relative to any scrolling. Given that, it may also be outside the
        // visible range, at which point we'll use a None value.
        let relative_cursor = match view.visible_region(buf)
                                        .relative_position(buf.cursor.line) {
            Visibility::Visible(line) => {
                Some(Position {
                    line: line,
                    offset: buf.cursor.offset,
                })
            }
            _ => None,
        };

        // Get the selected range, relative to the scrolled buffer.
        let line_range = mode.to_range(&*buf.cursor);
        let relative_highlight = relative_range(
            view.visible_region(buf),
            &line_range
        );

        // Bundle up the presentable data.
        let data = BufferData {
            tokens: Some(visible_tokens),
            cursor: relative_cursor,
            highlight: Some(relative_highlight),
            line_count: line_count(&buf.data()),
            scrolling_offset: line_offset,
        };

        // Handle cursor updates.
        view.set_cursor(data.cursor);

        // Draw the visible set of tokens to the terminal.
        view.draw_buffer(&data);

        // Draw the status line.
        view.draw_status_line(&vec![
            StatusLineData {
                content: " SELECT LINE ".to_string(),
                style: None,
                background_color: Some(Color::Blue),
                foreground_color: Some(Color::White),
            },
            buffer_status_line_data(&buf)
        ]);
    } else {
        // There's no buffer; clear the cursor.
        view.set_cursor(None);
    }

    // Render the changes to the screen.
    view.present();
}

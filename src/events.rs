use crate::{actions, App, TerminalFrame};
use crossterm::event::KeyCode;
use tui::layout::Rect;

/// Input events handler
pub fn handle_events(app: &mut App, keycode: KeyCode) {
    match keycode {
        KeyCode::Esc => actions::exit_app(app),
        KeyCode::Up => actions::list_up(app),
        KeyCode::Down => actions::list_down(app),
        KeyCode::Char(c) => actions::filter_add_char(app, c),
        KeyCode::Backspace => actions::filter_del_char(app),
        KeyCode::Enter => actions::run_ssh_session(app),
        _ => {}
    }
}

/// Handle cursor when typing
pub fn handle_input_cursor(app: &App, frame: &mut TerminalFrame, chunks: &[Rect]) {
    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    frame.set_cursor(
        // Put cursor past the end of the input text
        chunks[1].x + app.filter.len() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[1].y + 1,
    )
}

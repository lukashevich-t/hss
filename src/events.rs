use crate::{actions, App, TerminalFrame};
use crossterm::event::{KeyCode, KeyEvent};
use tui::layout::Rect;

/// Input events handler
pub fn handle_events(event: KeyEvent, app: &mut App) {
    handle_normal_events(app, event.code);
}

/// When user is viewing quests
fn handle_normal_events(app: &mut App, keycode: KeyCode) {
    // let keybindings = &app.configs.keybindings;

    if keycode == KeyCode::Esc {
        actions::exit_app(app);
    } else if keycode == KeyCode::Up {
        actions::list_up(app);
    } else if keycode == KeyCode::Down {
        actions::list_down(app);
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

use crossterm::event::KeyCode;
use tui::style::Color;

/// Application configs
#[derive(Default, Debug)]
pub struct Configs {
    pub colors: Colors,
}

#[derive(Debug)]
pub struct Colors {
    pub foreground: Color,
    pub background: Color,
    pub selection_fg: Color,
    pub selection_bg: Color,
    pub check_sign: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            selection_fg: Color::Black,
            selection_bg: Color::Yellow,
            check_sign: Color::Green,
        }
    }
}

/// Converts a `KeyCode` to `String`
pub fn keycode_to_string(keycode: KeyCode) -> String {
    let temp;

    let stringified = match keycode {
        KeyCode::Backspace => "Backspace",
        KeyCode::Enter => "Enter",
        KeyCode::Left => "←",
        KeyCode::Right => "→",
        KeyCode::Up => "↑",
        KeyCode::Down => "↓",
        KeyCode::Home => "Home",
        KeyCode::End => "End",
        KeyCode::PageUp => "Page Up",
        KeyCode::PageDown => "Page Down",
        KeyCode::Tab => "Tab",
        KeyCode::BackTab => "Back Tab",
        KeyCode::Delete => "Delete",
        KeyCode::Insert => "Insert",
        KeyCode::F(n) => {
            temp = format!("F{}", n);
            temp.as_str()
        }
        KeyCode::Char(char) => {
            temp = char.to_string();
            temp.as_str()
        }
        KeyCode::Null => "Null",
        KeyCode::Esc => "Esc",
    }
    .to_string();

    stringified
}

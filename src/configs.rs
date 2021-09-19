use crossterm::event::KeyCode;
use tui::style::Color;
use std::fs::File;
use std::io::Read;

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
        KeyCode::PageUp => "PageUp",
        KeyCode::PageDown => "PageDown",
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

pub fn read_host_names() -> Vec<String> {
    let mut home_dir = dirs::home_dir().expect("Cannot get user home directory!");
    home_dir.push(".ssh");
    home_dir.push("config");
    let display = home_dir.display();

    let mut file = match File::open(&home_dir) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    let mut host_names: Vec<String> = Vec::new();
    for line in s.split(&['\n', '\r'][..]) {
        let split = line.trim().split_whitespace().collect::<Vec<&str>>();
        if split.len() < 2 {
            continue;
        }
        let host_name = split[1].trim();
        if split[0].to_lowercase() == "host" && host_name.len() > 0 && host_name != "*" {
            host_names.push(String::from(host_name));
        }
    }
    host_names
}

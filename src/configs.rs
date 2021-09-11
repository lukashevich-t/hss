use crossterm::event::KeyCode;
use tui::style::Color;
use regex::Regex;
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

fn extract_host_names(config_lines: Vec<String>) -> Vec<String> {
    let separator = Regex::new(r"[ \t]+").unwrap();
    let v= config_lines;
    v.into_iter()
        .map(|x| String::from(separator.split(&x).into_iter().collect::<Vec<&str>>()[1].trim()))
        .collect::<Vec<String>>()
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
        Ok(_) => {},
    }
    let configs = s.split(&['\n', '\r'][..])
        .filter(|x| x.starts_with("Host "))
        .map(|x| String::from(x))
        .collect::<Vec<String>>();
    if configs.len() == 0 {
        println!("No configured hosts in {}", display);
        std::process::exit(1);
    }
    return extract_host_names(configs);
}

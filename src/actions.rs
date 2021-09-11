//! Actions to do after a specific event occurs

use crate::{App};

pub fn new_quest(app: &mut App) {
    app.selected_host = None;
}

pub fn exit_app(app: &mut App) {
    app.should_exit = true;
}

pub fn list_up(app: &mut App) {
    if let Some(index) = app.selected_host {
        if index > 0 {
            app.selected_host = Some(index - 1);
        }
    }
}

pub fn list_down(app: &mut App) {
    if let Some(index) = app.selected_host {
        if index < app.hosts.len() - 1 {
            app.selected_host = Some(index + 1);
        }
    }
}

pub fn input_add_char(app: &mut App, c: char) {
    app.input.push(c);
}

pub fn input_del_char(app: &mut App) {
    app.input.pop();
}

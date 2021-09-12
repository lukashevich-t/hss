//! Actions to do after a specific event occurs
use crate::App;
use std::cmp::min;

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
        if index < app.filtered_hosts.len() - 1 {
            app.selected_host = Some(index + 1);
        }
    }
}

pub fn list_home(app: &mut App) {
    app.selected_host = if app.filtered_hosts.len() > 0 { Some(0) } else { None };
}

pub fn list_pgup(app: &mut App) {
    // app.selected_host = if app.filtered_hosts.len() > 0 { Some(0) } else { None };
}

pub fn list_pgdn(app: &mut App) {
    print!("{}", app.host_list_client_height);
    let host_count = app.filtered_hosts.len();
    if host_count == 0 { return; };
    let selected_host_index = if let Some(i) = app.selected_host { i } else { 0 };
    app.selected_host = Some(min(host_count, selected_host_index + app.host_list_client_height as usize) - 1);
}

pub fn list_end(app: &mut App) {
    app.selected_host = if app.filtered_hosts.len() > 0 { Some(app.filtered_hosts.len() - 1) } else { None };
}

pub fn filter_add_char(app: &mut App, c: char) {
    app.filter.push(c);
    filter_hosts(app)
}

fn filter_hosts(app: &mut App) {
    let filter_string = app.filter.trim();
    app.filtered_hosts = app.hosts
        .iter()
        .filter(|x| x.contains(filter_string))
        .map(|x| String::from(x))
        .collect();
    app.selected_host = if app.filtered_hosts.len() == 0 { None } else { Some(0) }
}

pub fn filter_del_char(app: &mut App) {
    app.filter.pop();
    filter_hosts(app);
}

pub fn run_ssh_session(app: &mut App) {
    if let Some(index) = app.selected_host {
        app.should_exit = true;
        let host = app.filtered_hosts.get(index).unwrap();
        app.host_to_connect = Some(host.to_string());
    }
}

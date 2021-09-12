//! Custom widgets

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};
use crate::{configs::keycode_to_string, App};
use crossterm::event::KeyCode;

/// Split terminal view
pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
                .as_ref(),
        )
        .split(area);

    chunks
}

/// Shows a list of hosts
pub fn host_list(app: &mut App, height: u16) -> List {
    let height = (height - 2) as usize;
    app.first_visible_host_index = if let Some(selected_index) = app.selected_host {
        if selected_index < app.first_visible_host_index {
            selected_index
        } else if selected_index >= app.first_visible_host_index + height {
            selected_index.saturating_sub(height - 1)
        } else {
            app.first_visible_host_index
        }
    } else {
        0
    };

    let mut hosts = Vec::new();
    for q in app.filtered_hosts.iter().enumerate().skip(app.first_visible_host_index) {
        hosts.push(indexed_host_item(app, q))
    };
    // Map hosts to ListItem widget
    // app
    //     .filtered_hosts
    //     .iter()
    //     .enumerate()
    //     // .skip(app.first_visible_host)
    //     .map(|q| indexed_host_item(app, q))
    //     .collect();

    List::new(hosts).style(app.default_style()).block(
        Block::default()
            .title("SSH hosts")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(app.default_style()),
    )
}

/// Check if host is selected then renders it properly
fn indexed_host_item<'a>(app: &'a App, (index, host_name): (usize, &String)) -> ListItem<'a> {
    if let Some(selected_index) = app.selected_host {
        host_item(
            host_name.clone(),
            selected_index == index,
            app,
        )
    } else {
        host_item(host_name.clone(), false, app)
    }
}

/// Widget to show a single host
fn host_item(title: String, selected: bool, app: &App) -> ListItem {
    let style = if selected {
        app.selection_style()
    } else {
        app.default_style()
    };

    let host = ListItem::new(Spans::from(vec![
        Span::styled("   ", style),
        Span::styled(title, style),
    ]));

    host.style(style)
}

/// Input field for filter
pub fn filter_input(app: &App) -> Paragraph {
    let style = app.default_style();

    let filter = Paragraph::new(app.filter.as_ref()).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Filter")
            .border_type(BorderType::Rounded)
            .style(style),
    );

    filter
}

/// Help text
pub fn navigation_hint(app: &App) -> Paragraph {
    let msg = vec![
        Span::styled(
            keycode_to_string(KeyCode::Esc),
            app.default_style().add_modifier(Modifier::BOLD),
        ),
        Span::styled(": exit | ", app.default_style()),
        Span::styled(
            keycode_to_string(KeyCode::Enter),
            app.default_style().add_modifier(Modifier::BOLD),
        ),
        Span::styled(": run config | ", app.default_style()),
        Span::styled(
            format!(
                "{}/{}/{}/{}",
                keycode_to_string(KeyCode::Up),
                keycode_to_string(KeyCode::Down),
                keycode_to_string(KeyCode::PageUp),
                keycode_to_string(KeyCode::PageDown),
            ),
            app.default_style().add_modifier(Modifier::BOLD),
        ),
        Span::styled(": navigate", app.default_style()),
    ];
    let style = app.default_style();

    let mut help_text = Text::from(Spans::from(msg));
    help_text.patch_style(style);
    Paragraph::new(help_text).style(app.default_style())
}

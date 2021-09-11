//! Custom widgets

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};
use crate::{configs::keycode_to_string, App, Quest};
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

/// Shows a list of quests
pub fn quest_list(app: &App) -> List {
    // Map quests to ListItem widget
    let quests: Vec<ListItem> = app
        .quests
        .iter()
        .enumerate()
        .map(|q| indexed_quest_item(app, q))
        .collect();

    List::new(quests).style(app.default_style()).block(
        Block::default()
            .title("Quests")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(app.default_style()),
    )
}

/// Check if a quest is selected then renders it properly
fn indexed_quest_item<'a>(app: &'a App, (index, quest): (usize, &Quest)) -> ListItem<'a> {
    if let Some(selected_index) = app.selected_quest {
        quest_item(
            quest.title.clone(),
            quest.completed,
            selected_index == index,
            app,
        )
    } else {
        quest_item(quest.title.clone(), quest.completed, false, app)
    }
}

/// Widget to show a single quest
fn quest_item(title: String, completed: bool, selected: bool, app: &App) -> ListItem {
    let style = if selected {
        app.selection_style()
    } else {
        app.default_style()
    };

    let quest = if completed {
        ListItem::new(Spans::from(vec![
            Span::styled("✔  ", app.check_sign_style(selected)),
            Span::styled(title, app.checked_quest_style(selected)),
        ]))
    } else {
        ListItem::new(Spans::from(vec![
            Span::styled("   ", style),
            Span::styled(title, style),
        ]))
    };

    quest.style(style)
}

/// Input field to make a new quest
pub fn quest_input(app: &App) -> Paragraph {
    let style = app.default_style();

    let input = Paragraph::new(app.input.as_ref()).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Filter")
            .border_type(BorderType::Rounded)
            .style(style),
    );

    input
}

/// Help text
pub fn navigation_hint(app: &App) -> Paragraph {
    // let keybindings = &app.configs.keybindings;

    let msg = vec![
        Span::styled(
            keycode_to_string(KeyCode::Esc),
            app.default_style().add_modifier(Modifier::BOLD),
        ),
        Span::styled(" exit | ", app.default_style()),
        Span::styled(
            keycode_to_string(KeyCode::Enter),
            app.default_style().add_modifier(Modifier::BOLD),
        ),
        Span::styled(" run config | ", app.default_style()),
        Span::styled(
            format!(
                "{}/{}",
                keycode_to_string(KeyCode::Up),
                keycode_to_string(KeyCode::Down)
            ),
            app.default_style().add_modifier(Modifier::BOLD),
        ),
        Span::styled(" navigate list", app.default_style()),
    ];
    let style = app.default_style();

    let mut help_text = Text::from(Spans::from(msg));
    help_text.patch_style(style);
    Paragraph::new(help_text).style(app.default_style())
}

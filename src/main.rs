use crossterm::{
    event::{read, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use hss::{
    events::{handle_events},
    widget, App, CrossTerminal, DynResult, TerminalFrame,
};
use std::{error::Error, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};
use hss::configs::{Configs, read_host_names};
use std::process::Command;

fn main() -> DynResult {
    let mut terminal = initialize_terminal()?;

    // let hosts= (1..41).map(|x|  x.to_string()).collect();
    let hosts = read_host_names();
    let configs = Configs::default();
    let mut app = App::new(hosts, configs);

    draw_ui(&mut terminal, &mut app)?;
    cleanup_terminal(terminal)?;
    if let Some(host) = app.host_to_connect {
        let mut cmd = Command::new("ssh");
        cmd.arg(host);
        match cmd.status() {
            _ => {}
        }
    }
    Ok(())
}

/// Setup terminal
fn initialize_terminal() -> Result<CrossTerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

/// Draw user interface to terminal
fn draw_ui(terminal: &mut CrossTerminal, app: &mut App) -> DynResult {
    while !app.should_exit {
        terminal.draw(|f| {
            app_view(f, app);
        })?;

        if let Event::Key(event) = read()? {
            handle_events(app, event.code);
        }
    }

    Ok(())
}

/// Return terminal to it's normal state
fn cleanup_terminal(mut terminal: CrossTerminal) -> DynResult {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

/// A single frame of application view
fn app_view(frame: &mut TerminalFrame, app: &mut App) {
    let main_chunks = widget::main_chunks(frame.size());

    let host_list_area = main_chunks[0];
    app.host_list_client_height = host_list_area.height - 2;
    let host_list = widget::host_list(app);
    frame.render_widget(host_list, host_list_area);

    let filter_input = widget::filter_input(app);
    frame.render_widget(filter_input, main_chunks[1]);

    let navigation_hint = widget::navigation_hint(app);
    frame.render_widget(navigation_hint, main_chunks[2]);
}

pub mod actions;
pub mod configs;
pub mod events;
pub mod widget;

use configs::Configs;
use std::{error::Error, io::Stdout};
use tui::{
    backend::CrosstermBackend,
    style::{Style},
    Frame, Terminal,
};

pub type DynResult = Result<(), Box<dyn Error>>;
pub type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;
pub type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Application state
pub struct App {
    /// filter string
    pub filter: String,
    /// All ssh hosts
    pub hosts: Vec<String>,
    /// Filtered ssh hosts
    pub filtered_hosts: Vec<String>,
    /// Should be true when application wants to exit
    pub should_exit: bool,
    /// Current selected host
    pub selected_host: Option<usize>,
    /// Application Configs
    pub configs: Configs,
    /// SSH host to connect to after exit
    pub host_to_connect: Option<String>,
}

impl App {
    pub fn new(hosts: Vec<String>, configs: Configs) -> Self {
        let filtered_hosts = hosts.clone();
        Self {
            hosts,
            filtered_hosts,
            selected_host: Some(0),
            filter: String::new(),
            should_exit: false,
            configs,
            host_to_connect: None
        }
    }

    pub fn default_style(&self) -> Style {
        Style::default()
            .fg(self.configs.colors.foreground)
            .bg(self.configs.colors.background)
    }

    pub fn selection_style(&self) -> Style {
        self.default_style()
            .fg(self.configs.colors.selection_fg)
            .bg(self.configs.colors.selection_bg)
    }
}

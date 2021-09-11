
mod ssh_actions;
mod ssh_configs;

use dirs;
use std::fs::File;
use std::io::Read;
use std::process::Command;

fn main() {
    ssh_actions::enlist();
}



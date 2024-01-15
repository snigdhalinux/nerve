use std::vec;

use crate::args::TerminalSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_snigdha_terminal(terminal_setup: TerminalSetup){
    log::debug!("Installing -> {:?}", terminal_setup);
    match terminal_setup {
        TerminalSetup::Alacritty => install_snigdha_alacritty(),
        TerminalSetup::Foot => install_snigdha_foot(),
        TerminalSetup::GnomeTerminal => install_snigdha_gnome_terminal(),
        TerminalSetup::Kitty => install_snigdha_kitty(),
        TerminalSetup::Konsole => install_snigdha_konsole(),
        TerminalSetup::Terminator => install_snigdha_terminator(),
        TerminalSetup::Xfce => install_snigdha_xfce4_terminal(),
        TerminalSetup::Xterm => install_snigdha_xterm(),
        TerminalSetup::None => log::debug!("None")
    }
}

fn install_snigdha_alacritty(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-alacritty-config",
    ]);
}

fn install_snigdha_foot(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-foot-config",
    ]);
}

fn install_snigdha_gnome_terminal(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-gnome-terminal",
    ]);
}

fn install_snigdha_kitty(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-kitty-config",
    ]);
}

fn install_snigdha_konsole(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-konsole-config",
    ]);
}

fn install_snigdha_terminator(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-terminator-config",
    ]);
}

fn install_snigdha_xfce4_terminal(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-xfce4-terminal-config",
    ]);
}

fn install_snigdha_xterm(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-xterm-config",
    ]);
}
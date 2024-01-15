use crate::args::ShellSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_snigdha_shells(shell_setup: ShellSetup){
    log::debug!("Installing -> {:?}", shell_setup);
    match shell_setup {
        ShellSetup::Bash => install_snigdha_bash(),
        ShellSetup::Fish => install_snigdha_fish(),
        ShellSetup::Zsh => install_snigdha_zsh(),
        ShellSetup::None => log::debug!("None"),
    }
}

fn install_snigdha_bash(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-root-config",
    ]);
}

fn install_snigdha_fish(){
    install(PackageManager::Pacman, vec![
        "snigdhaos=fish-config",
    ]);
}

fn install_snigdha_zsh(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-zsh-config",
    ]);
}


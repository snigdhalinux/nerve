use crate::args::PassManSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_password_manager_setup(passman_setup:PassManSetup){
    log::debug!("Installing {:?}", passman_setup);
    match passman_setup {
        PassManSetup::Bitwarden => install_snigdha_bitwarden(),
        PassManSetup::Enpass => install_snigdha_enpass(),
        PassManSetup::Keepassxc => install_snigdha_keepassxc(),
        PassManSetup::Lastpass => install_snigdha_lastpass(),
        PassManSetup::None => log::debug!("No Password Manager Setup Selected!"),
    }
}

fn install_snigdha_bitwarden(){
    install(PackageManager::Pacman, vec![
        "bitwarden",
    ]);
}

fn install_snigdha_enpass(){
    install(PackageManager::Pacman, vec![
        "enpass-bin",
    ]);
}

fn install_snigdha_keepassxc(){
    install(PackageManager::Pacman, vec![
        "keepassxc",
    ]);
}

fn install_snigdha_lastpass(){
    install(PackageManager::Pacman, vec![
        "lastpass",
    ]);
}
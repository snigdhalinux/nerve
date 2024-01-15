use crate::args::DMSetup;
use crate::args::PackageManager;
use crate::internal::*;

use self::services::enable_snigdha_services;

pub fn install_snigdha_desktopmanagers(dm_setup: DMSetup){
    log::debug!("Installing -> {:?}", dm_setup);
    match dm_setup {
        DMSetup::Gdm => install_snigdha_gdm(),
        DMSetup::Sddm => install_snigdha_sddm(),
        DMSetup::None => log::debug!("None")
    }
}

fn install_snigdha_gdm(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-gdm-config",
    ]);
    enable_snigdha_services("gdm");
}

fn install_snigdha_sddm(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-sddm-config",
    ]);
    enable_snigdha_services("sddm")
}
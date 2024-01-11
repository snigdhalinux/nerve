use crate::args::DMSetup;
use crate::args::PackageManager;
use crate::internal::{files, files_eval, install};
use crate::internal::services::enable_service;

pub fn install_dm_setup(dm_setup: DMSetup){
    log::debug!("Installing {:?}", dm_setup);
    match dm_setup{
        DMSetup::None => log::debug!("No display manager setup selected!"),
    }
}

fn install_gdm(){
    install(PackageManager::Pacman, vec![
        "gdm",
    ]);
    enable_service("gdm");
}

fn install_sddm(){
    install(PackageManager::Pacman, vec![
        "sddm",
    ]);
    enable_service("sddm");
}

fn install_lightdm(){
    install(PackageManager::Pacman, vec![
        "sddm",
    ]);
    enable_service("lightdm")
}


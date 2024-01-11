use crate::args::FMSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_fm_setup(fm_setup: FMSetup){
    log::debug!("Installing {:?}", dm_setup);
    match fm_setup{
        FMSetup::Dolphin => install_snigdha_dolphin(),
        FMSetup::Nautilus => install_snigdha_nautilus(),
        FMSetup::Caja => install_snigdha_caja(),
        FMSetup::DoubleCMD => install_snigdha_doublecmd_qt5(),
        FMSetup::Thunar => install_snigdha_thunar(),
        FMSetup::None => log::debug!("No File Manager setup selected!"),
    }
}

fn install_snigdha_dolphin(){
    install(PackageManager::Pacman, vec![
        "dolphin",
    ]);
}

fn install_snigdha_nautilus(){
    install(PackageManager::Pacman, vec![
        "nautilus",
    ]);
}

fn install_snigdha_caja(){
    install(PackageManager::Pacman, vec![
        "caja",
    ]);
}

fn install_snigdha_doublecmd_qt5(){
    install(PackageManager::Pacman, vec![
        "doublecmd-qt5",
    ]);
}

fn install_snigdha_thunar(){
    install(PackageManager::Pacman, vec![
        "thunar",
        // "thunar-archive-plugin",
    ]);
}
use crate::args::PackageManager;
use crate::args::IdeSetup;
use crate::internal::*;

pub fn install_snigdha_ide(ide_setup: IdeSetup){
    log::debug!("Installing -> {:?}", ide_setup);
    match ide_setup {
        IdeSetup::Vscode => install_snigdha_vscode(),
        IdeSetup::Vscodium => install_snigdha_vscodium(),
        IdeSetup::PycharmPro => install_snigdha_pycharmpro(),
        IdeSetup::PycharmComm => install_snigdha_pycharmcomm(),
        IdeSetup::PycharmEAP => install_snigdha_pycharmeap(),
        IdeSetup::Clion => install_snigdha_clion(),
        IdeSetup::IntellijIDEAPro => install_snigdha_ideapro(),
        IdeSetup::IntellijIDEAComm => install_snigdha_intellijcomm(),
        IdeSetup::IntellijIDEAEAP => install_snigdha_intellijeap(),
        IdeSetup::IntellijIdeaCa => install_snigdha_intellijca(),
        IdeSetup::None => log::debug!("None"),
    }
}

fn install_snigdha_vscode(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-vscode-config",
    ]);
}

fn install_snigdha_vscodium(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-vscodium-config",
    ]);
}

fn install_snigdha_pycharmpro(){
    install(PackageManager::Pacman, vec![
        "pycharm-professional",
    ]);
}

fn install_snigdha_pycharmcomm(){
    install(PackageManager::Pacman, vec![
        "pycharm-community-jre",
    ]);
}

fn install_snigdha_pycharmeap(){
    install(PackageManager::Pacman, vec![
        "pycharm-community-eap"
    ]);
}

fn install_snigdha_clion(){
    install(PackageManager::Pacman, vec![
        "clion",
    ]);
}

fn install_snigdha_ideapro(){
    install(PackageManager::Pacman, vec![
        "intellij-idea-ultimate-edition",
        "intellij-idea-ultimate-edition-jre",
    ]);
}

fn install_snigdha_intellijcomm(){
    install(PackageManager::Pacman, vec![
        "intellij-idea-community-edition-jre",
    ]);
}

fn install_snigdha_intellijeap(){
    install(PackageManager::Pacman, vec![
        "intellij-idea-ue-eap",
    ]);
}

fn install_snigdha_intellijca(){
    install(PackageManager::Pacman, vec![
        "intellij-idea-ce-eap",
    ]);
}
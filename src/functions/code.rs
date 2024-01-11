use crate::args::CodeSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_code_setup(code_setup:CodeSetup){
    log::debug!("Installing {:?}", code_setup);
    match code_setup{
        CodeSetup::VSCode => install_snigdha_vs_code(),
        CodeSetup::VSCodium => install_snigdha_vscodium(),
        CodeSetup::PycharmProfessional => install_snigdha_pycharm_professional(),
        CodeSetup::PycharmEAP => install_snigdha_pycharm_eap(),
        CodeSetup::PycharmCommunity => install_snigdha_pycharm_community(),
        CodeSetup::Clion => install_snigdha_clion(),
        CodeSetup::ClionEAP => install_snigdha_clion_eap(),
        CodeSetup::Rustrover => install_snigdha_rustrover(),
        CodeSetup::RustroverEAP => install_snigdha_rustrover_eap(),
        CodeSetup::PhpStorm => install_snigdha_phpstorm(),
        CodeSetup::PhpStormEAP => install_snigdha_phpstorm_eap(),
        CodeSetup::Idea => install_snigdha_idea_ultimate(),
        CodeSetup::IdeaCommunity => install_snigdha_idea_community(),
        CodeSetup::None => log::debug!("No Ide Setup Selected!"),
    }
}

fn install_snigdha_vs_code(){
    install(PackageManager::Pacman, vec![
        "visual-studio-code-bin",
    ]);
}

fn install_snigdha_vscodium(){
    install(PackageManager::Pacman, vec![
        "vscodium-bin",
    ]);
}

fn install_snigdha_pycharm_professional(){
    install(PackageManager::Pacman, vec![
        "pycharm-professional",
    ]);
}

fn install_snigdha_pycharm_eap(){
    install(PackageManager::Pacman, vec![
        "pycharm-eap",
    ]);
}

fn install_snigdha_pycharm_community(){
    install(PackageManager::Pacman, vec![
        "pycharm-community-eap",
    ]);
}

fn install_snigdha_clion(){
    install(PackageManager::Pacman, vec![
        "clion",
    ]);
}

fn install_snigdha_clion_eap(){
    install(PackageManager::Pacman, vec![
        "clion-eap",
    ]);
}

fn install_snigdha_rustrover(){
    install(PackageManager::Pacman, vec![
        "rustrover",
    ]);
}

fn install_snigdha_rustrover_eap(){
    install(PackageManager::Pacman, vec![
        "rustrover-eap",
    ]);
}

fn install_snigdha_phpstorm(){
    install(PackageManager::Pacman, vec![
        "phpstorm",
    ]);
}

fn install_snigdha_phpstorm_eap(){
    install(PackageManager::Pacman, vec![
        "phpstorm-eap",
    ]);
}

fn install_snigdha_idea_ultimate(){
    install(PackageManager::Pacman, vec![
        "intellij-idea-ultimate-edition",
    ]);
}

fn install_snigdha_idea_community(){
    install(PackageManager::Pacman, vec![
        "intellij-idea-community-edition-jre",
    ]);
}
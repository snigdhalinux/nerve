use crate::args::CommunicationSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_communication_setup(communication_setup:CommunicationSetup){
    log::debug!("Installing {:?}", communication_setup);
    match communication_setup {
        CommunicationSetup::Element => install_snigdha_element_desktop(),
        CommunicationSetup::Discord => install_snigdha_discord(),
        CommunicationSetup::Signal => intsall_snigdha_signal(),
        CommunicationSetup::Slack => install_snigdha_slack(),
        CommunicationSetup::Teams => install_snigdha_teams(),
        CommunicationSetup::Viber => install_snigdha_viber(),
        CommunicationSetup::WhatsApp => install_snigdha_whatsapp(),
        CommunicationSetup::Telegram => install_snigdha_telegram(),
        CommunicationSetup::Wire => install_snigdha_wire(),
        CommunicationSetup::Zoom => install_snigdha_zoom(),
        CommunicationSetup::None => log::debug!("No Communication Setup Selected!"),
    }
}

fn install_snigdha_element_desktop(){
    install(PackageManager::Pacman, vec![
        "element-desktop",
    ]);
}

fn install_snigdha_discord(){
    install(PackageManager::Pacman, vec![
        "discord",
    ]);
}

fn intsall_snigdha_signal(){
    install(PackageManager::Pacman, vec![
        "signal-desktop",
    ]);
}

fn install_snigdha_slack(){
    install(PackageManager::Pacman, vec![
        "slack-desktop",
    ]);
}

fn install_snigdha_teams(){
    install(PackageManager::Pacman, vec![
        "teams",
    ]);
}

fn install_snigdha_viber(){
    install(PackageManager::Pacman, vec![
        "viber",
    ]);
}

fn install_snigdha_whatsapp(){
    install(PackageManager::Pacman, vec![
        "whatsapp-nativefier",
    ]);
}

fn install_snigdha_telegram(){
    install(PackageManager::Pacman, vec![
        "telegram-desktop-bin",
    ]);
}

fn install_snigdha_wire(){
    install(PackageManager::Pacman, vec![
        "wire-desktop",
    ]);
}

fn install_snigdha_zoom(){
    install(PackageManager::Pacman, vec![
        "zoom",
    ]);
}
use crate::args::BrowserSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_browser_setup(browser_setup: BrowserSetup){
    log::debug!("Installing{:?}", browser_setup);
    match browser_setup{
        BrowserSetup::Brave => install_snigdha_brave(),
        BrowserSetup::Chrome => install_snigdha_google_chrome(),
        BrowserSetup::FireFox => install_snigdha_firefox(),
        BrowserSetup::TorBrowser => install_snigdha_tor_browser(),
        BrowserSetup::Edge => install_snigdha_ms_edge(),
        BrowserSetup::Thorium => install_snigdha_thorium(),
        BrowserSetup::UnGoogledChromium => install_snigdha_ungoogled_chromium(),
        BrowserSetup::None => log::debug!("No Browser Selected!")
    }
}

fn install_snigdha_brave(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-brave-config"
    ]);
}

fn install_snigdha_google_chrome(){
    install(PackageManager::Pacman, vec![
        "google-chrome"
    ]);
}

fn install_snigdha_firefox(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-firefox-config"
    ]);
}

fn install_snigdha_tor_browser(){
    install(PackageManager::Pacman, vec![
        "tor-browser-bin"
    ]);
}

fn install_snigdha_ms_edge(){
    install(PackageManager::Pacman, vec![
        "microsoft-edge-stable-bin"
    ]);
}

fn install_snigdha_thorium(){
    install(PackageManager::Pacman, vec![
        "thorium-browser-bin"
    ]);
}

fn install_snigdha_ungoogled_chromium(){
    install(PackageManager::Pacman, vec![
        "ungoogled-chromium"
    ]);
}
use crate::args::PackageManager;
use crate::args::BrowserSetup;
use crate::internal::*;

pub fn snigdha_install_browser(browser_setup: BrowserSetup){
    log::debug!("Installing -> {:?}", browser_setup);
    match browser_setup{
        BrowserSetup::Brave => install_snigdha_brave(),
        BrowserSetup::FireFox => install_snigdha_firefox(),
        BrowserSetup::Chrome => install_snigdha_chrome(),
        BrowserSetup::Chromium => install_snigdha_chromium(),
        BrowserSetup::Tor => install_snigdha_tor(),
        BrowserSetup::WaterFox => install_snigdha_waterfox(),
        BrowserSetup::None => log::debug!("None"),
    }
}

fn install_snigdha_brave(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-brave-config",
    ]);
}

fn install_snigdha_firefox(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-firefox-config",
    ]);
}

fn install_snigdha_chrome(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-chrome-config",
    ]);
}

fn install_snigdha_chromium(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-chromium-config",
    ]);
}

fn install_snigdha_tor(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-tor-config",
    ]);
}

fn install_snigdha_waterfox(){
    install(PackageManager::Pacman, vec![
        "snigdhaos-waterfox-config"
    ]);
}
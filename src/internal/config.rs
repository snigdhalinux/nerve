use crate::args;
use crate::args::{DesktopSetup, DMSetup, ShellSetup, BrowserSetup, TerminalSetup, IdeSetup, GitSetup, PartitionMode, PackageManager};
use crate::functions::*;
use crate::internal::*;
// use crate::internal::exec::*;
// use crate::internal::files::sed_file;
// use crate::internal::secure;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;


#[derive(Serialize, Deserialize)]
struct Config {
    partition: Partition,
    bootloader: Bootloader,
    locale: Locale,
    networking: Networking,
    users: Vec<Users>,
    rootpass: String,
    desktop: String,
    displaymanager: String,
    browser: String,
    terminal: String,
    ide: String,
    git: String,
    snapper: bool,
    flatpak: bool,
    zramd: bool,
    extra_packages: Vec<String>,
    kernel: String,
}

#[derive(Serialize, Deserialize)]
struct Partition {
    device: String,
    mode: PartitionMode,
    efi: bool,
    partitions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Bootloader {
    r#type: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
struct Locale {
    locale: Vec<String>,
    keymap: String,
    timezone: String,
}

#[derive(Serialize, Deserialize)]
struct Networking {
    hostname: String,
    ipv6: bool,
}

#[derive(Serialize, Deserialize)]
struct Users {
    name: String,
    password: String,
    hasroot: bool,
    shell: String,
}

pub fn read_config(configpath: PathBuf) {
    let data = std::fs::read_to_string(&configpath);
    match &data {
        Ok(_) => {
            log::debug!("[ \x1b[2;1;32mOK\x1b[0m ] Read config file {configpath:?}");
        }
        Err(e) => {
            crash(
                format!("Read config file {configpath:?}  ERROR: {}", e),
                e.raw_os_error().unwrap(),
            );
        }
    }
    let config: std::result::Result<Config, serde_json::Error> =
        serde_json::from_str(&data.unwrap());
    match &config {
        Ok(_) => {
            log::debug!("[ \x1b[2;1;32mOK\x1b[0m ] Parse config file {configpath:?}",);
        }
        Err(e) => {
            crash(format!("Parse config file {configpath:?}  ERROR: {}", e), 1);
        }
    }
    let config: Config = config.unwrap();
    log::info!("Block device to use : /dev/{}", config.partition.device);
    log::info!("Partitioning mode : {:?}", config.partition.mode);
    log::info!("Partitioning for EFI : {}", config.partition.efi);
    let mut partitions: Vec<args::Partition> = Vec::new();
    for partition in config.partition.partitions {
        partitions.push(args::Partition::new(
            partition.split(':').collect::<Vec<&str>>()[0].to_string(),
            partition.split(':').collect::<Vec<&str>>()[1].to_string(),
            partition.split(':').collect::<Vec<&str>>()[2].to_string(),
        ));
    }
    let device = PathBuf::from("/dev/").join(config.partition.device.as_str());
    partition::partition(
        device,
        config.partition.mode,
        config.partition.efi,
        &mut partitions,
    );
    println!();
    base::install_base_packages();
    println!();
    log::info!("Adding Locales -> {:?}", config.locale.locale);
    locale::set_locale(config.locale.locale.join(" "));
    log::info!("Keymap -> {:?}", config.locale.keymap);
    locale::set_keyboard(config.locale.keymap.as_str());
    log::info!("Timezon -> {:?}", config.locale.timezone);
    locale::set_timezone(config.locale.timezone.as_str());
    println!();
    base::install_snigdha_packages(config.kernel);
    base::genfstab();
    println!();
    log::info!("Installing bootloader : {}", config.bootloader.r#type);
    log::info!("Installing bootloader to : {}", config.bootloader.location);
    if config.bootloader.r#type == "grub-efi" {
        base::install_bootloader_efi(PathBuf::from(config.bootloader.location));
    } else if config.bootloader.r#type == "grub-legacy" {
        base::install_bootloader_legacy(PathBuf::from(config.bootloader.location));
    }
    println!();
    log::info!("Hostname : {}", config.networking.hostname);
    log::info!("Enabling ipv6 : {}", config.networking.ipv6);
    network::set_hostname(config.networking.hostname.as_str());
    network::create_hosts();
    if config.networking.ipv6 {
        network::enable_ipv6();
    }
    println!();
    println!("---------");
    log::info!("Enabling zramd : {}", config.zramd);
    if config.zramd {
        base::install_zram();
    }
    println!();
    log::info!("Installing Desktop -> {:?}", config.desktop);
    match config.desktop.to_lowercase().as_str() {
        "onyx" => desktops::install_desktop_setup(DesktopSetup::Onyx),
        "kde" => desktops::install_desktop_setup(DesktopSetup::Kde),
        "plasma" => desktops::install_desktop_setup(DesktopSetup::Kde),
        "mate" => desktops::install_desktop_setup(DesktopSetup::Mate),
        "gnome" => {
            desktops::install_desktop_setup(DesktopSetup::Gnome);
            disable_xsession("gnome.desktop");
            disable_xsession("gnome-classic.desktop");
            disable_xsession("gnome-classic-xorg.desktop");
            disable_wsession("gnome.desktop");
            disable_wsession("gnome-wayland.desktop");
            disable_wsession("gnome-classic-xorg.desktop");
            disable_wsession("gnome-classic-wayland.desktop");
        },
        "cinnamon" => desktops::install_desktop_setup(DesktopSetup::Cinnamon),
        "xfce" => desktops::install_desktop_setup(DesktopSetup::Xfce),
        "budgie" => desktops::install_desktop_setup(DesktopSetup::Budgie),
        "enlightenment" => desktops::install_desktop_setup(DesktopSetup::Enlightenment),
        "lxqt" => desktops::install_desktop_setup(DesktopSetup::Lxqt),
        "sway" => desktops::install_desktop_setup(DesktopSetup::Sway),
        "i3" => desktops::install_desktop_setup(DesktopSetup::I3),
        "herbstluftwm" => desktops::install_desktop_setup(DesktopSetup::Herbstluftwm),
        "awesome" => desktops::install_desktop_setup(DesktopSetup::Awesome),
        "bspwm" => desktops::install_desktop_setup(DesktopSetup::Bspwm),
        "none/diy" => desktops::install_desktop_setup(DesktopSetup::None),
        _ => log::info!("No desktop setup selected!"),
    }
    println!();
    log::info!("Installing Display Managers -> {:?}", config.displaymanager);
    match config.displaymanager.to_lowercase().as_str() {
        "gdm" => {
            displaymangers::install_snigdha_desktopmanagers(DMSetup::Gdm);
            if !config.desktop.contains("gnome"){
                files::rename_file("/mnt/usr/lib/udev/rules.d/61-gdm.rules", "/mnt/usr/lib/udev/rules.d/61-gdm.rules.bak");
                disable_xsession("gnome.desktop");
                disable_xsession("gnome-xorg.desktop");
                disable_wsession("gnome.desktop");
                disable_wsession("gnome-wayland.desktop");
            }
            else {
                files_eval(
                    files::sed_file(
                        "/mnt/etc/gdm/custom.conf",
                        ".*WaylandEnable=.*",
                        "WaylandEnable=false"
                    ),
                    "Diasble -> WayLand!"
                );
            }
        },
        "sddm" => displaymangers::install_snigdha_desktopmanagers(DMSetup::Sddm),
        _ => log::info!("No DM Selected!"),
    }
    println!();
    log::info!("Installing Browser _> {:?}", config.browser);
    match config.browser.to_lowercase().as_str() {
        "brave" => {
            browsers::snigdha_install_browser(BrowserSetup::Brave);
        },
        "firefox" => {
            browsers::snigdha_install_browser(BrowserSetup::FireFox);
        },
        "chrome" => {
            browsers::snigdha_install_browser(BrowserSetup::Chrome);
        },
        "chromium" => {
            browsers::snigdha_install_browser(BrowserSetup::Chromium);
        },
        "tor" => {
            browsers::snigdha_install_browser(BrowserSetup::Tor);
        },
        "waterfox" => {
            browsers::snigdha_install_browser(BrowserSetup::WaterFox);
        },
        _ => log::info!("None!"),
    }
    println!();
    log::info!("Installing Terminal -> {:?}", config.terminal);
    let mut term_choice = String::new();
    match config.terminal.to_lowercase().as_str() {
        "alacritty" => {
            term_choice = String::from("alacritty");
            terminals::install_snigdha_terminal(TerminalSetup::Alacritty);
        },
        "foot" => {
            term_choice = String::from("foot");
            terminals::install_snigdha_terminal(TerminalSetup::Foot);
        },
        "gnome-terminal" => {
            term_choice = String::from("gnome-terminal");
            terminals::install_snigdha_terminal(TerminalSetup::GnomeTerminal);
        },
        "kitty" => {
            term_choice = String::from("kitty");
            terminals::install_snigdha_terminal(TerminalSetup::Kitty);
        },
        "konsole" => {
            term_choice = String::from("konsole");
            terminals::install_snigdha_terminal(TerminalSetup::Konsole);
        },
        "xfce" => {
            term_choice = String::from("xfce");
            terminals::install_snigdha_terminal(TerminalSetup::Xfce);
        },
        "xterm" => {
            term_choice = String::from("xterm");
            terminals::install_snigdha_terminal(TerminalSetup::Xterm);
        },
        _ => log::info!("None!"),
    }
    println!();
    log::info!("Installing IDE -> {:?}", config.ide);
    let mut ide_choice = String::new();
    match config.ide.to_lowercase().as_str() {
        "vscode" => {
            ide_choice = String::from("vscode");
            ide::install_snigdha_ide(IdeSetup::Vscode);
        },
        "vscodium" => {
            ide_choice = String::from("vscodium");
            ide::install_snigdha_ide(IdeSetup::Vscodium);
        },
        //dhur bal er gaan 
        "pycharm-pro" => {
            ide_choice = String::from("pycharm-pro");
            ide::install_snigdha_ide(IdeSetup::PycharmPro);
        },
        "pycharm-comm" => {
            ide_choice = String::from("pycharm-comm");
            ide::install_snigdha_ide(IdeSetup::PycharmComm);
        },
        "pycharm-eap" => {
            ide_choice = String::from("pycharm-eap");
            ide::install_snigdha_ide(IdeSetup::PycharmEAP);
        },
        "clion" => {
            ide_choice = String::from("clion");
            ide::install_snigdha_ide(IdeSetup::Clion);
        },
        "intellij-idea-ultimate" => {
            ide_choice = String::from("intellij-idea-ultimate");
            ide::install_snigdha_ide(IdeSetup::IntellijIDEAPro);
        },
        "intellij-idea-community" => {
            ide_choice = String::from("intellij-idea-community");
            ide::install_snigdha_ide(IdeSetup::IntellijIDEAComm);
        },
        "intellij-idea-eap" => {
            ide_choice = String::from("intellij-idea-eap");
            ide::install_snigdha_ide(IdeSetup::IntellijIDEAEAP);
        },
        "intellij-idea-ca" => {
            ide_choice = String::from("intellij-idea-ca");
            ide::install_snigdha_ide(IdeSetup::IntellijIdeaCa);
        },
        _ => log::info!("None!")
    }
    println!();
    log::info!("Installing Git Client -> {:?}", config.git);
    let mut git_choice = String::new();
    match config.git.to_lowercase().as_str() {
        "gitahead" => {
            git_choice = String::from("gitahead");
            git::install_snigdha_git(GitSetup::GitAhead);
        },
        "gitfiend" => {
            git_choice = String::from("gitfiend");
            git::install_snigdha_git(GitSetup::GitFiend);
        },
        "gitkraken" => {
            git_choice = String::from("gitkraken");
            git::install_snigdha_git(GitSetup::GitKraken);
        },
        "github-desktop" => {
            git_choice = String::from("github-desktop");
            git::install_snigdha_git(GitSetup::GithubDesktop);
        },
        "gittyup" => {
            git_choice = String::from("gittyup");
            git::install_snigdha_git(GitSetup::GittyUp);
        },
        "megit" => {
            git_choice = String::from("megit");
            git::install_snigdha_git(GitSetup::Megit);
        },
        "smartgit" => {
            git_choice = String::from("smartgit");
            git::install_snigdha_git(GitSetup::SmartGit);
        },
        _ => log::info!("None!"),
    }
    log::info!("Installing -> Snapper! {:?}", config.snapper);
    if config.snapper{
        base::snigdha_snapper();
    }
    println!();
    log::info!("Enabling flatpak : {}", config.flatpak);
    if config.flatpak {
        base::install_flatpak();
    }
    log::info!("Extra packages : {:?}", config.extra_packages);
    let mut extra_packages: Vec<&str> = Vec::new();
    for i in 0..config.extra_packages.len() {
        extra_packages.push(config.extra_packages[i].as_str());
    }
    install(PackageManager::Pacman, extra_packages);
    log::info!("Enable System Services!");
    base::enable_system_services();
    for i in 0..config.users.len() {
        log::info!("Creating user : {}", config.users[i].name);
        // log::info!("Setting use password : {}", config.users[i].password);
        log::info!("Enabling root for user : {}", config.users[i].hasroot);
        log::info!("Setting user shell : {}", config.users[i].shell);
        match config.users[i].shell.to_lowercase().as_str() {
            "bash" => shells::install_snigdha_shells(ShellSetup::Bash),
            "fish" => shells::install_snigdha_shells(ShellSetup::Fish),
            "zsh" => shells::install_snigdha_shells(ShellSetup::Zsh),
            _ => log::info!("None!"),
        }
        users::new_user(
            config.users[i].name.as_str(),
            config.users[i].hasroot,
            config.users[i].password.as_str(),
            false,
            "bash"
        );
        println!("---------");
    }
    println!();
    // log::info!("Setting root password : {}", config.rootpass);
    users::root_pass(config.rootpass.as_str());
    println!();
    files::copy_file("/tmp/nerve.log", "/mnt/var/log/nerve.log");
    println!("Installation finished! You may reboot now!")
}


fn disable_xsession(session: &str){
    log::debug!("Disabling -> {}", session);
    files::rename_file(&("/mnt/usr/share/xsessions/".to_owned()+session), &("/mnt/usr/share/xsessions/".to_owned()+session+".disable"));
}

fn disable_wsession(session: &str){
    log::debug!("Disabling -> {}", session);
    files::rename_file(&("/mnt/usr/share/wayland-sessions/".to_owned()+session), &("/mnt/usr/share/wayland-sessions/".to_owned()+session+".disable"));
}
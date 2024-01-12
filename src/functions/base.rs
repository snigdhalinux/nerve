use crate::args::PackageManager;
use crate::internal::exec::*;

// use crate::internal::files::append_file;
use crate::internal::*;
use log::warn;
use std::path::PathBuf;

pub fn install_base_packages() {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    init_snigdha_keyring();
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
    install::install(PackageManager::Pacstrap, vec![
        "base",
        "glibc-locales",
        "snigdhaos-mirrorlist",
        "chaotic-mirrorlist",
        "rate-mirrors",
        "snigdhaos-keyring",
        "chaotic-keyring",
        "archlinux-keyring",
    ]);
    files::copy_file("/etc/pacman.d/mirrorlist", "/mnt/etc/pacman.d/mirrorlist");
    get_snigdha_fastest_mirrors();
}

pub fn install_packages(){

}

pub fn genfstab() {
    exec_eval(
        exec(
            "bash",
            vec![
                String::from("-c"),
                String::from("genfstab -U /mnt >> /mnt/etc/fstab"),
            ],
        ),
        "Generate fstab",
    );
}

pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec![
        "grub",
        "efibootmgr",
        "crystal-grub-theme",
        "os-prober",
        "crystal-branding",
    ]);
    let efidir = std::path::Path::new("/mnt").join(efidir);
    let efi_str = efidir.to_str().unwrap();
    if !std::path::Path::new(&format!("/mnt{efi_str}")).exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=crystal"),
                String::from("--removable"),
            ],
        ),
        "install grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=crystal"),
            ],
        ),
        "install grub as efi without --removable",
    );
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\"",
        ),
        "enable crystal grub theme",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    install::install(vec![
        "grub",
        "crystal-grub-theme",
        "os-prober",
        "crystal-branding",
    ]);
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![String::from("--target=i386-pc"), device],
        ),
        "install grub as legacy",
    );
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\"",
        ),
        "enable crystal grub theme",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn setup_timeshift() {
    install(vec!["timeshift", "timeshift-autosnap", "grub-btrfs"]);
    exec_eval(
        exec_chroot("timeshift", vec![String::from("--btrfs")]),
        "setup timeshift",
    )
}

pub fn install_homemgr() {
    install(vec!["nix"]);
}

pub fn install_flatpak() {
    install(vec!["flatpak"]);
    exec_eval(
        exec_chroot(
            "flatpak",
            vec![
                String::from("remote-add"),
                String::from("--if-not-exists"),
                String::from("flathub"),
                String::from("https://flathub.org/repo/flathub.flatpakrepo"),
            ],
        ),
        "add flathub remote",
    )
}

pub fn install_zram() {
    install(vec!["zram-generator"]);
    files::create_file("/mnt/etc/systemd/zram-generator.conf");
    files_eval(
        files::append_file("/mnt/etc/systemd/zram-generator.conf", "[zram0]"),
        "Write zram-generator config",
    );
}

fn init_snigdha_keyring(){
    log::info!("Upgrading Keyrings...");
    exec_eval(
        exec(
            "rm",
            vec![
                String::from("-rf"),
                String::from("/etc/pacman.d/gnupg"),
            ],
        ),
        "Removing Keys...",
    );
    exec_eval(
        exec(
            "pacman-key",
            vec![
                String::from("--init"),
            ],
        ),
        "Initiating Keys...",
    );
    exec_eval(
        exec(
            "pacman-key",
            vec![
                String::from("--populate"),
            ],
        ),
        "Populating Keys...",
    );
    //refreshing Arch Mirrorlist using rate-mirrors
    exec_eval(
        exec(
            "rate-mirrors",
            vec![
                String::from("--concurrency"),
                String::from("40"),
                String::from("--disable-comments"),
                String::from("--allow-root"),
                String::from("--save"),
                String::from("/etc/pacman.d/mirrorlist"),
                String::from("arch"),
            ],
        ),
        "Getting fastest mirrors for Arch Linux...",
    );
}

fn get_snigdha_fastest_mirrors(){
    log::info!("Getting Fastest Chaotic Aur Mirrorlist From Your Location...");
    exec_eval(
        exec_chroot(
            "rate-mirrors",
            vec![
                String::from("--concurrency"),
                String::from("40"),
                String::from("--disable-comments"),
                String::from("--allow-root"),
                String::from("--save"),
                String::from("/etc/pacman.d/mirrorlist"),
                String::from("chaotic-aur"),
            ],
        ),
        "Setting Fastest Chaotic AUR Mirrors...",
    );
}
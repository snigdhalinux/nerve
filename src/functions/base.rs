use crate::args::PackageManager;
use crate::internal::exec::*;
use crate::internal::files::append_file;
use crate::internal::*;
use crate::internal::services::enable_snigdha_services;
use log::warn;
use std::path::PathBuf;
// use serde_json::Value::String;

pub fn install_base_packages(kernel: String) {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    init_snigdha_keyrings();
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
    install::install(PackageManager::Pacstrap, vec![
        "base",
        "glibc-locales",
        "rate-mirors",
        "snigdhaos-keyring",
        // "snigdhaos-mirrorlist",
        "chaotic-keyring",
        "chaotic-mirrorlist",
    ]);
    files::copy_file("/etc/pacman.d/mirrorlist", "/mnt/etc/pacman.d/mirrorlist");
    get_snigdha_fastest_chaotic();
}

pub fn install_snigdha_packages(kernel: String){
    let kernel_to_install = if kernel.is_empty(){
        "linux-zen"
    }
    else {
        match kernel.as_str() {
            "linux" => "linux",
            "linux-zen" => "linux-zen",
            "linux-lts" => "linux-lts",
            _ => {
                warn!("Unknown Kernel: {}, Using Linux-Zen Instead!", kernel);
                "linux-zen"
            }
        }
    };
    install::install(PackageManager::Pacman, vec![
        kernel_to_install,
        format!("{kernel_to_install}-headers").as_str(),
        "linux-firmware",
        "accountservice",
        "alsa-utils",
        "arch-install-scripts",
        "broadcom-wl-=dkms",
        "dialog",
        "dhcpcd",
        "dosfstools",
        "edk2-shell",
        "inetutils",
        "irqbalance",
        "lvm2",
        "man-db",
        "man-pages",
        "memtest86+",
        "mesa",
        "mesa-utils",
        "mkinitcpio-nfs-utils",
        "mkinitcpio-openswap",
        "most",
        "mtools",
        "nano",
        "nbd",
        "networkmanager",
        "net-tools",
        "netctl",
        "ntp",
        "pavucontrol",
        "profile-sync-daemon",
        "pv",
        "rsync",
        "rtl8821cu-morrownr-dkms-git",
        "sof-firmware",
        "squashfs-tools",
        "sudo",
        "syslinux",
        "systemd-sysvcompat",
        "testdisk",
        "texinfo",
        "usbutils",
        "wget",
        "wireless_tools",
        "wpa_supplicant",
        "xfsprogs",

        //snigdha OS
        "snigdhaos-brave-config",
        "snigdhaos-google-chrome-config",
        "snigdhaos-chroumium-config",
        "snigdhaos-tor-config",
        "snigdhaos-system-config",
        "snigdhaos-neofetch-config",
        "snigdhaos-grub-theme",
        "snigdhaos-root",
        "snigdhaos-vscode-theme",
        // I will make other packages optional
    ]);
    hardware::snigdha_set_cores();
    hardware::snigdha_cpu_gpu_check(kernel_to_install);
    hardware::snigdha_virt_check();

    exec_eval(
        exec(
            "sed",
            vec![
                String::from("-i"),
                String::from("-e"), //need fucking hooks to edit
                String::from("s/^HOOKS=.*/HOOKS=(base systemd autodetect modconf kms block keyboard sd-vconsole lvm2 filesystems fsck)"),
                String::from("/mnt/etc/mkinitcpio.conf"),
            ],
        ),
        "-> Set Hooks!"
    );
    files::copy_file("/etc/skel/.bashrc", "/mnt/etc/skel/.bashrc");
    files::copy_file("/mnt/usr/lib/os-release-snigdha", "/mnt/usr/lib/os-release");
    files::copy_file("/etc/grub.d/40_custom", "/mnt/etc/grub.d/40_custom");
    files::copy_file("/etc/NetworkManager/system-connections", "/mnt/etc/NetworkManager/system-connections");

    files_eval(
        files::sed_file(
            "/mnt/etc/mkinitcpio.conf",
            "#COMPRESSION=\"zstd\"",
            "COMPRESSION=\"zstd\"",
        ),
        "SET COMP --> ZSTD",
    );

    //nsswitch.conf
    files_eval(
        files::sed_file(
            "/mnt/etc/nsswitch.conf",
            "hosts:.*",
            "hosts: mymachines resolve [!UNAVAIL=return] files dns mdns wins myhostname",
        ),
        "--> NSSWITCH CONFIG"
    );
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
    install::install(PackageManager::Pacman, vec![
                "grub",
                "efibootmgr",
                "snigdhaos-grub-theme",
                "os-prober",
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
                String::from("--bootloader-id=GRUB"),
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
                String::from("--bootloader-id=GRUB"),
            ],
        ),
        "install grub as efi without --removable",
    );
    snigdha_grub_params();
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/usr/share/grub/themes/snigdhaos-grub-theme/theme.txt\"",
        ),
        "enable snigha os grub theme",
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
    install::install(PackageManager::Pacman, vec![
                "grub",
                "snigdhaos-grub-theme",
                "os-prober",
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
    snigdha_grub_params();
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

// pub fn setup_timeshift() {
//     install(, vec!["timeshift", "timeshift-autosnap", "grub-btrfs"]);
//     exec_eval(
//         exec_chroot("timeshift", vec![String::from("--btrfs")]),
//         "setup timeshift",
//     )
// }

pub fn install_homemgr() {
    install(PackageManager::Pacman, vec!["nix"]);
}

pub fn install_flatpak() {
    install(PackageManager::Pacman, vec!["flatpak"]);
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
    install(PackageManager::Pacman, vec!["zram-generator"]);
    files::create_file("/mnt/etc/systemd/zram-generator.conf");
    files_eval(
        files::append_file("/mnt/etc/systemd/zram-generator.conf", "[zram0]"),
        "Write zram-generator config",
    );
}

fn init_snigdha_keyrings(){
    log::info!("Upgrading Your Host Keyrings!");
    exec_eval(
        exec(
            "rm",
            vec![
                String::from("-rf"),
                String::from("/etc/pacman.d/gnupg"),
            ],
        ),
        "Removing...",
    );
    exec_eval(
        exec(
            "pacman-key",
            vec![
                String::from("--init"),
            ],
        ),
        "Init Keys!",
    );
    exec_eval(
        exec(
            "pacman-key",
            vec![
                String::from("--populate"),
            ],
        ),
        "Populate Keys!",
    );
    //arch linux mirrorlist.... -> not using reflector
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
        "Set Fastest Mirror For Arch Linux",
    );
}

fn get_snigdha_fastest_chaotic(){
    log::info!("Setting Chaotic Fastest Mirror!!");
    exec_eval(
        exec_chroot(
            "rate-mirrors", //we need chroot
            vec![
                String::from("--concurrency"),
                String::from("40"),
                String::from("--disable-comments"),
                String::from("--allow-root"),
                String::from("--save"),
                String::from("/etc/pacman.d/chaotic-mirrorlist"),
                String::from("chaotic-aur"),
            ],
        ),
        "Set Fastest Mirror For Chaotic AUR!",
    );
} //later on I will make snigdha-mirrorlist

pub fn snigdha_grub_params(){
    files_eval(
        files::sed_file(
            "/mnt/etc/default/grub",
            "GRUB_DISTRIBUTOR=.*",
            "GRUB_DISTRIBUTOR=\"Snigdha\"",
        ),
        "Setting -> Grub Parameters!"
    );

    files_eval(
        files::sed_file(
            "/mnt/etc/default/grub",
            "GRUB_CMDLINE_LINUX_DEFAULT=.*",
            "GRUB_CMDLINE_LINUX_DEFAULT=\"quiet loglevel=3 audit=0 nvme_load=yes zswap.enabled=0 fbcon=nodefer nowatchdog\"",
        ),
        "->Kernel Parameters"
    );

    files_eval(
        files::sed_file(
            "/mnt/etc/default/grub",
            "/#GRUB_DISABLE_OS_PROBER=.*",
            "GRUB_DISABLE_OS_PROBER=false",
        ),
        "Enable --> Dual Boot!"
    );
}

pub fn snigdha_snapper(){
    install(PackageManager::Pacman, vec![
        "btrfs-assistant",
        "btrfs-progs",
        "btrfsmaintenance",
        "grub-btrfs",
        "snap-pac",
        "snap-pac-grub",
        "snapper-support",
        "inotify-tools",
    ]);
    files_eval(
        files::sed_file(
            "/mnt/etc/default/grub-btrfs/config",
            "#GRUB_BTRFS_LIMIT=.*",
            "GRUB_BTRFS_LIMIT=\"5\"",
        ),
        "Grub BTRFS -> LIMIT!"
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/default/grub-btrfs/config",
            "#GRUB_BTRFS_SHOW_SNAPSHOTS_FOUND=.*",
            "GRUB_BTRFS_SHOW_SNAPSHOTS_FOUND=\"false\"",
        ),
        "Snapshots!!!!!!!!!!!!!!!!"
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/default/grub-btrfs/config",
            "#GRUB_BTRFS_SHOW_TOTAL_SNAPSHOTS_FOUND=.*",
            "GRUB_BTRFS_SHOW_TOTAL_SNAPSHOTS_FOUND=\"false\"",
        ),
        "TOTAL SNPASHOTS --> F!"
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/conf.d/snapper",
            "SNAPPER_CONFIGS=.*",
            "SNAPPER_CONFIGS=\"root\"",
        ),
        "TOTAL SNPASHOTS --> C!"
    );
    exec_eval(
        exec_chroot(
            "btrfs", vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("/.snapshots"),
            ],
        ),
        "/.snapshots -> btrfs sub!"
    );
    files::copy_file("/mnt/etc/snapper/config-templates/garda", "/mnt/etc/snapper/configs/root");
    enable_snigdha_services("grub-brtfsd");
}

pub fn snigdha_install_cuda(){
    install(PackageManager::Pacman, vec!["cuda"]);
}

pub fn enable_system_services(){
    enable_snigdha_services("bluetooth");
    enable_snigdha_services("cronie");
    enable_snigdha_services("irqalance");
    enable_snigdha_services("NetworkManager");
    enable_snigdha_services("systemd-timesyncd");
    enable_snigdha_services("vnstat");
}
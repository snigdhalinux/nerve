mod args;
mod functions;
mod internal;
mod logging;

use crate::args::{BootloaderSubcommand, Cli, Command, UsersSubcommand};
use crate::functions::*;
// use crate::internal::secure;
use clap::Parser;

fn main() {
    human_panic::setup_panic!();
    let cli: Cli = Cli::parse();
    println!("verbose: {}", cli.verbose);
    let log_file_path = "/tmp/nerve";
    logging::init(cli.verbose, log_file_path);
    match cli.command {
        Command::Partition(args) => {
            let mut partitions = args.partitions;
            partition::partition(
                args.device,
                args.mode,
                args.efi,
                &mut partitions,
            );
        }
        Command::InstallBase => {
            base::install_base_packages();
        }
        Command::Locale(args) => {
            locale::set_locale(args.locales.join(" "));
            locale::set_keyboard(&args.keyboard);
            locale::set_timezone(&args.timezone);
        }
        Command::InstallPackages(args) =>{
            base::install_snigdha_packages(args.kernel);
        }
        Command::GenFstab => {
            base::genfstab();
        }
        Command::SetupSnapper => base::snigdha_snapper(),
        Command::Bootloader { subcommand } => match subcommand {
            BootloaderSubcommand::GrubEfi { efidir } => {
                base::install_bootloader_efi(efidir);
            }
            BootloaderSubcommand::GrubLegacy { device } => {
                base::install_bootloader_legacy(device);
            }
        },
        
        Command::Networking(args) => {
            if args.ipv6 {
                network::create_hosts();
                network::enable_ipv6()
            } else {
                network::create_hosts();
            }
            network::set_hostname(&args.hostname);
        }
        Command::Zram => {
            base::install_zram();
        }
        Command::Users { subcommand } => match subcommand {
            UsersSubcommand::NewUser(args) => {
                users::new_user(
                    &args.username,
                    args.hasroot,
                    &args.password,
                    true,
                    &args.shell,
                );
            }
            UsersSubcommand::RootPass { password } => {
                users::root_pass(&password);
            }
        },
        Command::Nix => {
            base::install_homemgr();
        }
        Command::Flatpak => {
            base::install_flatpak();
        }
        Command::Config { config } => {
            crate::internal::config::read_config(config);
        }
        Command::Desktops { desktop } => {
            desktops::install_desktop_setup(desktop);
        }
        Command::DisplayManagers { displaymanager } => {
            displaymangers::install_snigdha_desktopmanagers(displaymanager);
        }
        Command::Shells { shell } => {
            shells::install_snigdha_shells(shell);
        }
        Command::Browsers { browser } => {
            browsers::snigdha_install_browser(browser);
        }
        Command::Terminals { terminal } => {
            terminals::install_snigdha_terminal(terminal);
        }
        Command::Ide { ide } => {
            ide::install_snigdha_ide(ide);
        }
        Command::Git { git } => {
            git::install_snigdha_git(git);
        }
        Command::EnableServices => {
            base::enable_system_services();
        }
    }
}

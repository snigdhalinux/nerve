use crate::args::PackageManager;
use log::{error, info, warn};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use clap::error::ContextValue::Strings;
use human_panic::print_msg;
use serde_json::Value::String;

pub fn install(pkgmanager: PackageManager, pkgs: Vec<&str>){
    let mut retry = Arc::new(Mutex::new(true));
    let mut retry_counter = 0;
    while *retry.lock().unwrap() && retry_counter < 15 {
        retry = Arc::new(Mutex::new(false));
        let retry_clone = Arc::clone(&retry);
        let mut pkgmanager_cmd = Command::new("true").spawn().expect("Failed To Initiate by 'true'");
        let mut pkgmanager_name = Strings.new();
        match pkgmanager {
            PackageManager::Pacman => {
                pkgmanager_cmd = Command::new("arch-chroot").arg("/mnt").arg("pacman").arg("-Syyu").arg("--needed").arg("--noconfirm").args(&pkgs).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("Failed To Start Pacman...");
                pkgmanager_name = String::from("pacman");
            },
            PackageManager::Pacstrap => {
                pkgmanager_cmd = Command::new("pacstrap").arg("/mnt").args(&pkgs).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("Failed To Start Pacstrap...");
                pkgmanager_name = String::from("pacstrap");
            },
            PackageManager::None => {
                log::debug!("No Setup Selected!");
            },

            // let stdout_handle =
        };
        let stdout_handle = pkgmanager_cmd.stdout.take().unwrap();
        let stedrr_handle = pkgmanager_cmd.stderr.take().unwrap();
        let stdout_thread = thread::spawn(move || {
            let reader = BufReader::new(stdout_handle);
            for line in reader.lines(){
                let line = line.expect("Failed to read stdout!");
                info!("{}", line);
            }
        });

        let exit_status = pkgmanager_cmd.wait().expect("Failed to initiate package manager!");
        let stedrr_thread = thread::spawn(move || {
            let reader = BufReader::new(stedrr_handle);
            for line in reader.lines(){
                if *retry_clone.lock().unwrap(){
                    break;
                }
                let line = line.expect("Failed to read stedrr!");
                let exit_code = exit_status.code().unwrap_or(-1);
                if exit_code == 0{
                    warn!(
                        "{} (exit code {}): {}", pkgmanager_name, exit_code, line
                    );
                }
                else {
                    error!(
                        "{} (exit code {}): {}", pkgmanager_name, exit_code, line
                    );
                }
                if line.contains("failed retrieving file") && line.contains("from"){
                    if let Some(mirror_name) = extract_mirror_name(&line){
                        if let Some(mirrorlist_file) = find_mirrorlist_file(&mirror_name, &pkgmanager_name){
                            if let Err(err) = move_server_line(&mirrorlist_file, &mirror_name){
                                error!(
                                    "Failed to move 'Server' line in {}: {}", mirrorlist_file, err
                                );
                            }
                            else { 
                                log::info!("Detected Unstable mirror: {}. Retrying with a new one...", mirror_name);
                                let mut retry = retry_clone.lock().unwrap();
                                *retry = true;
                            }
                        }
                    }
                }
                else if line.contains("signature from") && line.contains("is invalid") {
                    let package_name = extract_package_name(&line);
                    let repository = get_repository_name(&package_name);
                    println!("Package {} found in repository {}", package_name, repository);
                }
            }
        })
    }
}
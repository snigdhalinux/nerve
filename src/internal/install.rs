use crate::args::PackageManager;
use log::{error, info, warn};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
// use clap::error::ContextValue::Strings;
// use flexi_logger::ErrorChannel::File;
// use human_panic::print_msg;
// use serde_json::Value::String;

pub fn install(pkgmanager: PackageManager, pkgs: Vec<&str>){
    let mut retry = Arc::new(Mutex::new(true));
    let mut retry_counter = 0;
    while *retry.lock().unwrap() && retry_counter < 15 {
        retry = Arc::new(Mutex::new(false));
        let retry_clone = Arc::clone(&retry);
        let mut pkgmanager_cmd = Command::new("true").spawn().expect("Failed To Initiate by 'true'");
        let mut pkgmanager_name = String::new();
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
                    let mut mirrorlist_filename = String::new();
                    if pkgmanager_name == "pacstrap"{
                        if repository == "core" || repository =="extra" || repository == "community" || repository == "multilib"{
                            mirrorlist_filename = String::from("/etc/pacman.d/mirrorlist");
                        }
                        if repository == "chaotic-aur" {
                            String::from("/etc/pacman.d/chaotic-mirrorlist");
                        }
                    }
                    else if pkgmanager_name == "pacman" {
                        if repository == "core" || repository =="extra" || repository == "community" || repository == "multilib"{
                            mirrorlist_filename = String::from("/etc/pacman.d/mirrorlist");
                        }
                        if repository == "chaotic-aur" {
                            String::from("/etc/pacman.d/chaotic-mirrorlist");
                        }
                    }
                    match get_fastest_mirror_name(&mirrorlist_filename) {
                        Ok(mirror_name) => {
                            println!("Mirror Name: {}", mirror_name);
                            if let Err(err) = move_server_line(&mirrorlist_filename, &mirror_name){
                                error!(
                                    "Failed to move 'Server' line {}:{}",mirrorlist_filename, err
                                );
                            }
                            else {
                                log::info!(
                                    "Detected Invalid Signature {}. Retrying ...", mirror_name
                                );
                                let mut retry = retry_clone.lock().unwrap();
                                *retry = true;
                            }
                        }
                        Err(err) => eprintln!(
                            "Error: {}", err
                        ),
                    }
                }
            }
        });
        stdout_thread.join().expect("stdout thread panpicked");
        stedrr_thread.join().expect("stedrr thread panpicked");
        if !exit_status.success(){
            error!(
                "The package manager failed with exit code: {}", exit_status.code().unwrap_or(-1)
            );
        }
        retry_counter += 1;
    }
}

fn get_fastest_mirror_name(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines(){
        let line = line?;
        if let Some(eq_index) = line.find('='){
            let tr_line = line[..eq_index].trim();
            if tr_line == "Server"{
                let mir_url = line[eq_index + 1..].trim();
                return Ok(mir_url.to_string());
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Mirror Not Found!"))
}

fn get_repository_name(package_name: &str) -> String {
    let output = Command::new("pacman").arg("-Si").arg(package_name).output();
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8(output.stdout);
            match stdout {
                Ok(stdout) => {
                    if let Some(repo_line) = stdout.lines().find(|line|line.starts_with("Repository")){
                        let parts: Vec<&str> = repo_line.split(':').collect();
                        if parts.len() >= 2 {
                            return parts[1].trim().to_string();
                        }
                    }
                }
                Err(_) => eprintln!(
                    "Failed to transform -> String"
                ),
            }
        }
        Ok(_) => eprintln!("Package Not Found!"),
        Err(_) => eprintln!("Command Exec Failed!"),
    }
    String::new()
}

fn extract_package_name(input: &str) -> String {
    let err_prefix = "error:";
    let colon = ':';
    if let Some(error_idx) = input.find(err_prefix){
        let rem_text = &input[
            error_idx + err_prefix.len()..
        ];
        if let Some(colon_idx) = rem_text.find(colon) {
            let package_name = &rem_text[..colon_idx].trim();
            return package_name.to_string();
        }
    }
    String::new()
}

fn move_server_line(mirrorlist_path: &str, mirror_name: &str)-> io::Result<()> {
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(mirrorlist_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line?;
        lines.push(line);
    } //lets initiate last server index
    // Ok(())
    let last_server_index = lines.iter().rposition(|line|line.trim().starts_with("Server"));
    if let Some(last_server_index) = last_server_index{
        if let Some(mir_url_index) = lines.iter().position(|line|line.contains(mirror_name)){
            let mir_url_line = lines.remove(mir_url_index);
            let ins_index = last_server_index;
            lines.insert(ins_index, mir_url_line.clone());
            let mut file = OpenOptions::new().write(true).truncate(true).open(mirrorlist_path)?;
            for line in lines{
                writeln!(file, "{}", line)?;
            }
            log::info!("'{}' move -> {}", mir_url_line, mirrorlist_path);
        }
    }
    Ok(())
}

fn find_mirrorlist_file(mirror_name: &str, pkgmanager_name: &str) -> Option<String> {
    // todo!()
    let mut mirrorlist_paths: [&str; 2] = ["", ""];
    if pkgmanager_name == "pacstrap"{
        mirrorlist_paths = [
            "/etc/pacman.d/mirrorlist",
            "etc/pacman.d/chaotic-mirrorlist",
        ];
    }
    else if pkgmanager_name == "pacman" {
        mirrorlist_paths = [
            "/etc/pacman.d/mirrorlist",
            "etc/pacman.d/chaotic-mirrorlist",
        ];
    }
    for &mirrorlist_paths in &mirrorlist_paths {
        if let Ok(content) = fs::read_to_string(mirrorlist_paths){
            if content.contains(mirror_name){
                return Some(mirrorlist_paths.to_string());
            }
        }
    }
    None
}

fn extract_mirror_name(error_message: &str) -> Option<String> {
    let words: Vec<&str> = error_message.split_whitespace().collect();
    if let Some(from_index) = words.iter().position(|&word | word == "from"){
        if let Some(mirror_name) = words.get(from_index + 1){
            return Some(mirror_name.to_string());
        }
    }
    None
}




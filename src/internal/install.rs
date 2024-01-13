use crate::functions::partition::umount;
use crate::internal::*;
use std::process::Command;
use crate::args::PackageManager;

pub fn install(manager: PackageManager, pkgs: Vec<&str>) {
    exec_eval(
        Command::new("pacstrap").arg("/mnt").args(&pkgs).status(),
        format!("Install packages {}", pkgs.join(", ")).as_str(),
    );
    umount("/mnt/dev");
}

use std::fmt::format;
use std::process::Command;
use std::thread::available_parallelism;
use crate::args::PackageManager;
use crate::internal::{exec_eval, files, files_eval, install};
use crate::internal::exec::exec;
use crate::internal::files::sed_file;
use crate::internal::services::enable_snigdha_services;

pub fn snigdha_virt_check(){
    let output = Command::new("systemd-detect-virt").output().expect("Failed to initiate systemd-detect-virt");
    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
    result.pop();
    if result == "oracle"{
        install(PackageManager::Pacman, vec![
            "virtualbox-guest-utils"
        ]);
        //we need to make a service file to initiate..
        enable_snigdha_services("vboxservice");
    }
    else if result == "vmware" {
        install(PackageManager::Pacman, vec![
            "open-vm-tools",
            "xf86-video-firmware"
        ]);
        enable_snigdha_services("vmware-vmblock-fuse");
        enable_snigdha_services("vmtoolsd");
        exec_eval(
            exec(
                "sed", vec![
                    String::from("-i"),
                    String::from("-e"),
                    String::from("/"), //**** Module should be seen before writing!
                    String::from("-e"),
                    String::from("/mnt/etc/mkinitcpio.conf"),
                ],
            ),
            "Set VM Mods",
        );
    }
    else if result == "qemu" || result == "kvm" {
        install(PackageManager::Pacman, vec![
            "qemu-guest-agent",
            "spice-vdagent"
        ]);
        enable_snigdha_services("qemu-guest-agent");
    }
    else if result == "microsoft" {
        install(PackageManager::Pacman, vec![
            "hyperv",
            "xf86-video-fbdev"
        ]);
        enable_snigdha_services("hv_fcopy_daemon");
        enable_snigdha_services("hv_kvp_daemon");
        enable_snigdha_services("hv_vss_daemon");

        exec_eval(
            exec(
                "sed", vec![
                    String::from("-i"),
                    String::from("-e"),
                    String::from("/"), //**** Module should be seen before writing!
                    String::from("-e"),
                    String::from("/mnt/etc/default/grub"),
                ],
            ),
            "->HyperV"
        );
    }
}

pub fn snigdha_set_cores(){
    let def_param_appx = available_parallelism().unwrap().get();
    log::info!("Found : {} cores", def_param_appx);
    if def_param_appx > 1 {
        files_eval(
            sed_file(
                "/mnt/etc/makepkg.conf",
                "#MAKEFLAGS=.*",
                &(format!("MAKEFLAGS=\"-j{}\"", def_param_appx)),
            ),
            "SET FLAGS ->>"
        );
        files_eval(
            files::sed_file(
                "/mnt/etc/makepkg.conf",
                "#BUILDDIR=.*",
                "BUILDDIR=/tmp/makepkg",
            ),
            "Optimizing Compilation Time->",
        );
        //Compression Settings ---> Abhiraj @iconized will do it.
    }
}

pub fn snigdha_cpu_gpu_check(kernel: &str){
    //-->> CPU
    if snigdha_cpu_detect().contains("Intel"){
        log::info!("CPU-->INTEL");
        install(PackageManager::Pacman, vec![
            "intel-compute-runtime",
            "intel-ucode"
        ]);
    }
    else if snigdha_cpu_detect().contains("AMD") {
        log::info!("CPU-->AMD");
        install(PackageManager::Pacman, vec![
            "amd-ucode"
        ]);
    }

    //-->> GPU
    let snigdha_gpu_detect_out = Command::new("lspci").arg("-k").output().expect("Failed -> lspci");
    let snigdha_gpu_detect = String::from_utf8_lossy(&snigdha_gpu_detect_out.stdout);
    let mut snigdha_gpu_flag = false;
    if snigdha_gpu_detect.contains("AMD"){
        log::info!("GPU-->AMD");
        install(PackageManager::Pacman, vec![
            "xf86-video-amdgpu",
            "opencl-amd"
        ]);
        snigdha_gpu_flag = true;
    }

    if snigdha_gpu_detect.contains("ATI") && !snigdha_gpu_detect.contains("AMD"){
        log::info!("GPU--> ATI");
        install(PackageManager::Pacman, vec![
            "opencl-mesa"
        ]);
        snigdha_gpu_flag = true;
    }

    if snigdha_gpu_detect.contains("NVIDIA"){
        log::info!("GPU-->NVIDIA");
        if snigdha_gpu_detect.contains("GM107") || snigdha_gpu_detect.contains("GM108") || snigdha_gpu_detect.contains("GM200") || snigdha_gpu_detect.contains("GM204") || snigdha_gpu_detect.contains("GM206") || snigdha_gpu_detect.contains("GM20B"){
            log::info!("NVIDIA --> NV110 Family Detected");
            snigdha_gpu_flag = true;

            if kernel == "linux"{
                install(PackageManager::Pacman, vec![
                    "nvidia"
                ]);
            }
            else if kernel == "linux-lts" {
                install(PackageManager::Pacman, vec![
                    "nvidia-lts"
                ]);
            }
            else {
                install(PackageManager::Pacman, vec![
                    "nvidia-dkms"
                ]);
            }
            install(PackageManager::Pacman, vec![
                "nvidia-settings"
            ]);
        }
        //TU102 TU104 TU106 TU116 TU117
        if snigdha_gpu_detect.contains("TU102") || snigdha_gpu_detect.contains("TU104") || snigdha_gpu_detect.contains("106") || snigdha_gpu_detect.contains("116") || snigdha_gpu_detect.contains("TU117"){
            log::info!("NVIDIA--> NV160 Family Detected!");
            snigdha_gpu_flag = true;
            if kernel =="linux"{
                install(PackageManager::Pacman, vec![
                    "nvidia-open"
                ]);
            }
            else {
                install(PackageManager::Pacman, vec![
                    "nividia-open-dkms"
                ]);
            }
            install(PackageManager::Pacman, vec![
                "nvidia-settings"
            ]);
        }
        //GF100 GF108, GF106, GF104, GF110, GF114, GF116, GF117, GF119
        if snigdha_gpu_detect.contains("GF100") || snigdha_gpu_detect.contains("GF104") || snigdha_gpu_detect.contains("GF106") || snigdha_gpu_detect.contains("GF108") || snigdha_gpu_detect.contains("GF110") || snigdha_gpu_detect. contains("GF114") || snigdha_gpu_detect. contains("GF116") || snigdha_gpu_detect. contains("GF117") || snigdha_gpu_detect. contains("GF119") {
            log::info!("DETECTED -> NVDIA NVC0");
            snigdha_gpu_flag = true;
            install(PackageManager::Pacman, vec![
                "nvidia-340xx-dkms",
                "nvidia-340xx-settings"
            ]);
        }
        if snigdha_gpu_detect.contains("GK104") || snigdha_gpu_detect.contains("GK107") || snigdha_gpu_detect.contains("GK106") || snigdha_gpu_detect.contains("GK110") || snigdha_gpu_detect.contains("GK110B") || snigdha_gpu_detect. contains("GK208B") || snigdha_gpu_detect. contains("GK208") || snigdha_gpu_detect. contains("GK20A") || snigdha_gpu_detect. contains("GK210") {
            log::info!("DETECTED -> NVDIA NVE0");
            snigdha_gpu_flag = true;
            install(PackageManager::Pacman, vec![
                "nvidia-470xx-dkms",
                "nvidia-470xx-settings"
            ]);
        }
        
    }
}

fn snigdha_cpu_detect() -> String{
    let snigdha_cpu_output = Command::new("lscpu").output().expect("Failed -> lscpu!");
    let lscpu_str = std::str::from_utf8(&snigdha_cpu_output.stdout).expect("Failed -> Lscpu + utf8!");
    let ven_id_ln = lscpu_str.lines().find(|line| line.starts_with("Vendor ID:")).expect("!Found -> Vendor ID.");
    let ven_id = ven_id_ln.split(':').nth(1).expect("Invalid -> Vendor ID!").trim();
    ven_id.to_string()
}
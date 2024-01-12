use crate::internal::exec::*;
use crate::internal::*;

pub fn enable_snigdha_services(){
    log::debug("Enabling {}", dm); //4 displyamanagers
    exec_eval(
        exec_chroot(
            "systemctl",
            vec![
                String::from("enable"),
                String::from(dm),
            ]
        ),
        format!("Enable {}", dm).as_str(),
    );
}
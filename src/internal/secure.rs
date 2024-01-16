use crate::internal::*;

pub fn secure_snigdha_pass_config(){
    files_eval(
        files::sed_file(
            "/mnt/etc/login.defs",
            "PASS_MAX_DAYS	99999",
            "PASS_MAX_DAYS	365",
        ),
        "Pass Expires -> 365 Days",
    );
}

pub fn secure_snigdha_ssh_config(){
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#Port.*",
            "Port 2222",
        ),
        "SSH Port -> 2222",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#PermitRootLogin.*",
            "PermitRootLogin no",
        ),
        "Root login -> Disable",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#PubkeyAuthentication.*",
            "PubkeyAuthentication yes",
        ),
        "Pulic Key Auth -> True",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#PasswordAuthentication.*",
            "PasswordAuthentication no",
        ),
        "Password Auth -> False",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#PermitEmptyPasswords.*",
            "PermitEmptyPasswords no",
        ),
        "Empty Password -> False",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#IgnoreRhosts.*",
            "IgnoreRhosts yes",
        ),
        "Remot Host Access -> True",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#LoginGraceTime.*",
            "LoginGraceTime 30",
        ),
        "Secure Login Grace Time -> 30",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#MaxAuthTries.*",
            "MaxAuthTries 3",
        ),
        "Maximum Authentication -> 3",
    );
    files_eval(
        files::sed_file(
            "/mnt/etc/ssh/sshd_config",
            "#HostbasedAuthentication.*",
            "HostbasedAuthentication no",
        ),
        "Access Auth Via Rhosts -> False",
    );
    files_eval(
        files::append_file(
            "/mnt/etc/ssh/sshd_config",
            "Protocol 2"
        ),
        "Set  SSH Protocol!",
    );
}


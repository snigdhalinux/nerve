use crate::args::PackageManager;
use crate::args::GitSetup;
use crate::internal::*;

pub fn install_snigdha_git(git_setup: GitSetup){
    log::debug!("Installing -> {:?}", git_setup);
    match git_setup {
        GitSetup::GitAhead => install_snigdha_gitahead(),
        GitSetup::GitFiend => install_snigdha_gitfiend(),
        GitSetup::GitKraken => install_snigdha_gitkraken(),
        GitSetup::GithubDesktop => install_snigdha_github_desktop(),
        GitSetup::GittyUp => install_snigdha_gittyup(),
        GitSetup::Megit => install_snigdha_megit(),
        GitSetup::SmartGit => install_snigdha_smartgit(),
        GitSetup::None => log::debug!("None"),
    }
}

fn install_snigdha_github_desktop(){
    install(PackageManager::Pacman, vec![
        "github-desktop",
    ]);
}

fn install_snigdha_gitkraken(){
    install(PackageManager::Pacman, vec![
        "gitkraken",
    ]);
}

fn install_snigdha_smartgit(){
    install(PackageManager::Pacman, vec![
        "smartgit",
    ]);
}

fn install_snigdha_megit(){
    install(PackageManager::Pacman, vec![
        "megit",
    ]);
}

fn install_snigdha_gitahead(){
    install(PackageManager::Pacman, vec![
        "gitahead",
    ]);
}

fn install_snigdha_gitfiend(){
    install(PackageManager::Pacman, vec![
        "gitfiend",
    ]);
}

fn install_snigdha_gittyup(){
    install(PackageManager::Pacman, vec![
        "gittyup",
    ]);
}
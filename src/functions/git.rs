use crate::args::GitSetup;
use crate::args::PackageManager;
use crate::internal::*;

pub fn install_git_setup(git_setup:GitSetup){
    log::debug!("Installing {:?}", git_setup);
    match git_setup {
        GitSetup::Github => install_snigdha_github(),
        GitSetup::Gitkraken => install_snigdha_gitkraken(),
        GitSetup::Githead => install_snigdha_githead(),
        GitSetup::Gitg => install_snigdha_gitg(),
        GitSetup::Gitfriend => install_snigdha_gitfriend(),
        GitSetup::None => log::debug!("Not Git Client Setup Selected!"),
    }
}

fn install_snigdha_github(){
    install(PackageManager::Pacman, vec![
        "github-desktop-bin",
    ]);
}

fn install_snigdha_gitkraken(){
    install(PackageManager::Pacman, vec![
        "gitkraken",
    ]);
}

fn install_snigdha_githead(){
    install(PackageManager::Pacman, vec![
        "githead",
    ]);
}

fn install_snigdha_gitg(){
    install(PackageManager::Pacman, vec![
        "gitg",
    ]);
}

fn install_snigdha_gitfriend(){
    install(PackageManager::Pacman, vec![
        "gitfriend",
    ]);
}

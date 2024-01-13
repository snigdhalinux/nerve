pub mod config;
pub mod exec;
pub mod files;
pub mod install;
pub mod returncode_eval;
pub mod strings;
pub mod install;
mod hardware;
mod services;

pub use install::install;
pub use returncode_eval::*;
pub use strings::crash;

// Fuck the macro
pub use install::install;
pub use returncode_eval::*;
pub use strings::crash;

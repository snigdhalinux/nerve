use flexi_logger::{style, DeferredNow, LogSpecification, Logger, FileSpec, Duplicate};
use log::LevelFilter;
use std::fs;
use std::io::Write;
use crate::internal::files;

pub fn init(verbosity: u8, log_file_path: &str) {
    let log_specification = match verbosity {
        0 => LogSpecification::builder()
            .default(LevelFilter::Info)
            .build(),
        1 => LogSpecification::builder()
            .default(LevelFilter::Debug)
            .build(),
        _ => LogSpecification::builder()
            .default(LevelFilter::Trace)
            .build(),
    };
    if fs::metadata(log_file_path).is_ok(){
        files::remove_files(log_file_path) //need to create remove-files
    }
    Logger::with(log_specification).log_to_file(FileSpec::default().basename(log_file_path).suffix("log").suppress_timestamp(),).duplicate_to_stderr(Duplicate::All).format(format_log_entry).start().unwrap();
}

/// Formats a log entry with color
fn format_log_entry(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    let msg = record.args().to_string();
    let level = record.level();
    // let msg = apply_uwu(level, msg);
    let time = now.now().time();
    let time_str = time.format("%H:%M:%S").to_string();
    write!(
        w,
        "[ {} ] {} {}",
        style(level).paint(level.to_string()),
        time,
        msg
    )
}

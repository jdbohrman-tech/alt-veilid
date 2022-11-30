use backtrace::Backtrace;
use log::*;
use simplelog::*;
use std::fs::OpenOptions;
use std::panic;
use std::path::{Path, PathBuf};

pub fn veilid_core_setup<'a>(
    log_tag: &'a str,
    terminal_log: Option<Level>,
    file_log: Option<(Level, &Path)>,
) {
    if let Err(e) = veilid_core_setup_internal(log_tag, terminal_log, file_log) {
        panic!("failed to set up veilid-core: {}", e);
    }
}

fn veilid_core_setup_internal<'a>(
    _log_tag: &'a str,
    terminal_log: Option<Level>,
    file_log: Option<(Level, &Path)>,
) -> Result<(), String> {
    let mut logs: Vec<Box<dyn SharedLogger>> = Vec::new();

    let mut cb = ConfigBuilder::new();
    for ig in veilid_core::DEFAULT_LOG_IGNORE_LIST {
        cb.add_filter_ignore_str(ig);
    }

    if let Some(level) = terminal_log {
        logs.push(TermLogger::new(
            level.to_level_filter(),
            cb.build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
    }
    if let Some((level, log_path)) = file_log {
        let logfile = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(log_path)
            .map_err(|e| {
                format!(
                    "log open error: {} path={:?} all_dirs={:?}",
                    e,
                    log_path,
                    std::fs::read_dir(std::env::var("HOME").unwrap())
                        .unwrap()
                        .map(|d| d.unwrap().path())
                        .collect::<Vec<PathBuf>>()
                )
            })?;
        logs.push(WriteLogger::new(
            level.to_level_filter(),
            cb.build(),
            logfile,
        ))
    }
    CombinedLogger::init(logs).map_err(|e| format!("logger init error: {}", e))?;

    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();
        if let Some(location) = panic_info.location() {
            error!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        } else {
            error!("panic occurred but can't get location information...");
        }
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic payload: {:?}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic payload: {:?}", s);
        } else if let Some(a) = panic_info.payload().downcast_ref::<std::fmt::Arguments>() {
            error!("panic payload: {:?}", a);
        } else {
            error!("no panic payload");
        }
        error!("Backtrace:\n{:?}", bt);
    }));

    Ok(())
}

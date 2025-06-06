use crate::server::*;
use crate::settings::Settings;
use crate::tools::*;
use crate::veilid_logs::*;
use crate::*;
use futures_util::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_async_std::Signals;
use veilid_core::tools::*;

#[instrument(level = "trace", skip_all)]
async fn handle_signals(mut signals: Signals) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // XXX: reload configuration?
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                shutdown();
            }
            _ => unreachable!(),
        }
    }
}

pub async fn run_veilid_server_with_signals(
    settings: Settings,
    server_mode: ServerMode,
    veilid_logs: VeilidLogs,
) -> EyreResult<()> {
    // Catch signals
    let signals =
        Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT]).wrap_err("failed to init signals")?;
    let handle = signals.handle();

    let signals_task = spawn("signals", handle_signals(signals));

    // Run veilid server
    let res = run_veilid_server(settings, server_mode, veilid_logs).await;

    // Terminate the signal stream.
    handle.close();
    let _ = signals_task.await;

    res
}

#[warn(missing_docs)]
#[instrument(level = "trace", skip_all, err)]
pub fn run_daemon(settings: Settings, _args: CmdlineArgs) -> EyreResult<()> {
    let daemon = {
        let mut daemon = daemonize::Daemonize::new();
        let s = settings.read();
        if let Some(pid_file) = s.daemon.pid_file.clone() {
            daemon = daemon.pid_file(pid_file.clone());
        }
        if let Some(chroot) = &s.daemon.chroot {
            daemon = daemon.chroot(chroot);
        }
        if let Some(working_directory) = &s.daemon.working_directory {
            daemon = daemon.working_directory(working_directory);
        }
        if let Some(user) = &s.daemon.user {
            daemon = daemon.user(user.as_str());
        }
        if let Some(group) = &s.daemon.group {
            daemon = daemon.group(group.as_str());
        }

        let stdout_file = if let Some(stdout_file) = &s.daemon.stdout_file {
            Some(std::fs::File::create(stdout_file).wrap_err("Failed to create stdio file")?)
        } else {
            None
        };
        if let Some(stderr_file) = &s.daemon.stderr_file {
            if Some(stderr_file) == s.daemon.stdout_file.as_ref() {
                // same output file for stderr and stdout
                daemon = daemon.stderr(
                    stdout_file
                        .as_ref()
                        .unwrap()
                        .try_clone()
                        .wrap_err("Failed to clone stdout file")?,
                );
            } else {
                daemon = daemon.stderr(
                    std::fs::File::create(stderr_file).wrap_err("Failed to create stderr file")?,
                );
            }
        }
        if let Some(stdout_file) = stdout_file {
            daemon = daemon.stdout(stdout_file);
        }

        daemon
    };

    // Daemonize
    daemon.start().wrap_err("Failed to daemonize")?;

    // Now, run the server
    block_on(async {
        // Init combined console/file logger
        let veilid_logs = VeilidLogs::setup(settings.clone())?;

        run_veilid_server_with_signals(settings, ServerMode::Normal, veilid_logs).await
    })
}

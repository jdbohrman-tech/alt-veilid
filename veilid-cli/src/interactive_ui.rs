use std::io::Write;

use crate::command_processor::*;
use crate::cursive_ui::CursiveUI;
use crate::settings::*;
use crate::tools::*;
use crate::ui::*;

use console::{style, Term};
use flexi_logger::writers::LogWriter;
use rustyline_async::SharedWriter;
use rustyline_async::{Readline, ReadlineError, ReadlineEvent};
use stop_token::future::FutureExt as StopTokenFutureExt;
use stop_token::*;

pub type InteractiveUICallback = Box<dyn FnMut() + Send>;

pub struct InteractiveUIInner {
    cmdproc: Option<CommandProcessor>,
    stdout: Option<SharedWriter>,
    error: Option<String>,
    done: Option<StopSource>,
    connection_state_receiver: flume::Receiver<ConnectionState>,
    log_enabled: bool,
    enable_color: bool,
}

#[derive(Clone)]
pub struct InteractiveUI {
    inner: Arc<Mutex<InteractiveUIInner>>,
    _settings: Arc<Settings>,
}

impl InteractiveUI {
    pub fn new(settings: &Settings) -> (Self, InteractiveUISender) {
        let (cssender, csreceiver) = flume::unbounded::<ConnectionState>();

        let term = Term::stdout();
        let enable_color = console::colors_enabled() && term.features().colors_supported();

        // Create the UI object
        let this = Self {
            inner: Arc::new(Mutex::new(InteractiveUIInner {
                cmdproc: None,
                stdout: None,
                error: None,
                done: Some(StopSource::new()),
                connection_state_receiver: csreceiver,
                log_enabled: true,
                enable_color,
            })),
            _settings: Arc::new(settings.clone()),
        };

        let ui_sender = InteractiveUISender {
            inner: this.inner.clone(),
            connection_state_sender: cssender,
        };

        (this, ui_sender)
    }

    pub async fn command_loop(&self) {
        let (mut readline, mut stdout) =
            match Readline::new("> ".to_owned()).map_err(|e| e.to_string()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };

        let (connection_state_receiver, done) = {
            let inner = self.inner.lock();
            (
                inner.connection_state_receiver.clone(),
                inner.done.as_ref().unwrap().token(),
            )
        };

        self.inner.lock().stdout = Some(stdout.clone());

        CursiveUI::set_start_time();

        // Wait for connection to be established
        let done2 = done.clone();
        let self2 = self.clone();
        let mut stdout2 = stdout.clone();
        let connection_state_jh = spawn("connection state handler", async move {
            loop {
                match connection_state_receiver
                    .recv_async()
                    .timeout_at(done2.clone())
                    .await
                {
                    Ok(Ok(ConnectionState::ConnectedTCP(sa, st))) => {
                        let tstr = st
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|n| display_ts(n.as_micros() as u64))
                            .unwrap_or_else(|_| "???".to_string());
                        let _ = writeln!(stdout2, "Connected TCP: {} @ {}", sa, tstr);
                    }
                    Ok(Ok(ConnectionState::ConnectedIPC(pb, st))) => {
                        let tstr = st
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|n| display_ts(n.as_micros() as u64))
                            .unwrap_or_else(|_| "???".to_string());
                        let _ = writeln!(
                            stdout2,
                            "Connected IPC: {} @ {}",
                            pb.to_string_lossy(),
                            tstr
                        );
                    }
                    Ok(Ok(ConnectionState::RetryingTCP(sa, st))) => {
                        let tstr = st
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|n| display_ts(n.as_micros() as u64))
                            .unwrap_or_else(|_| "???".to_string());
                        let _ = writeln!(stdout2, "Retrying TCP: {} @ {}", sa, tstr);
                    }

                    Ok(Ok(ConnectionState::RetryingIPC(pb, st))) => {
                        let tstr = st
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|n| display_ts(n.as_micros() as u64))
                            .unwrap_or_else(|_| "???".to_string());
                        let _ =
                            writeln!(stdout2, "Retrying IPC: {} @ {}", pb.to_string_lossy(), tstr);
                    }
                    Ok(Ok(ConnectionState::Disconnected)) => {
                        let _ = writeln!(stdout2, "Disconnected");
                    }
                    Ok(Err(e)) => {
                        eprintln!("Error: {:?}", e);
                        self2.inner.lock().done.take();
                        break;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        loop {
            if let Some(e) = self.inner.lock().error.clone() {
                println!("Error: {:?}", e);
                break;
            }

            match readline.readline().timeout_at(done.clone()).await {
                Ok(Ok(ReadlineEvent::Line(line))) => {
                    let line = line.trim();

                    if !line.is_empty() {
                        readline.add_history_entry(line.to_string());
                    }

                    if line == "clear" {
                        if let Err(e) = readline.clear() {
                            println!("Error: {:?}", e);
                        }
                    } else if line == "log error" {
                        let opt_cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = opt_cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                "change_log_level api error",
                                UICallback::Interactive(Box::new(|| {})),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                            }
                        }
                    } else if line == "log warn" {
                        let opt_cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = opt_cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                "change_log_level api warn",
                                UICallback::Interactive(Box::new(|| {})),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                            }
                        }
                    } else if line == "log info" {
                        let opt_cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = opt_cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                "change_log_level api info",
                                UICallback::Interactive(Box::new(|| {})),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                            }
                        }
                    } else if line == "log debug" || line == "log" {
                        let opt_cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = opt_cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                "change_log_level api debug",
                                UICallback::Interactive(Box::new(|| {})),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                            }
                        }
                        if line == "log" {
                            self.inner.lock().log_enabled = true;
                        }
                    } else if line == "log trace" {
                        let opt_cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = opt_cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                "change_log_level api trace",
                                UICallback::Interactive(Box::new(|| {})),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                            }
                        }
                    } else if line == "log off" {
                        let opt_cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = opt_cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                "change_log_level api off",
                                UICallback::Interactive(Box::new(|| {})),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                            }
                        }
                    } else if line == "log hide" || line == "log disable" {
                        self.inner.lock().log_enabled = false;
                    } else if line == "log show" || line == "log enable" {
                        self.inner.lock().log_enabled = true;
                    } else if !line.is_empty() {
                        if line == "help" {
                            let _ = writeln!(
                                stdout,
                                r#"
Interactive Mode Commands:
    help - Display this help
    clear - Clear the screen
    log [level]      - Set the client api log level for the node to one of: error,warn,info,debug,trace,off
        hide|disable - Turn off viewing the log without changing the log level for the node
        show|enable  - Turn on viewing the log without changing the log level for the node
                     - With no option, 'log' turns on viewing the log and sets the level to 'debug'
"#
                            );
                        }

                        let cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = &cmdproc {
                            if let Err(e) = cmdproc.run_command(
                                line,
                                UICallback::Interactive(Box::new({
                                    //let mut stdout = stdout.clone();
                                    move || {
                                        // if let Err(e) = writeln!(stdout) {
                                        //     println!("Error: {:?}", e);
                                        // }
                                    }
                                })),
                            ) {
                                if let Err(e) = writeln!(stdout, "Error: {}", e) {
                                    println!("Error: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok(Ok(ReadlineEvent::Interrupted)) => {
                    break;
                }
                Ok(Ok(ReadlineEvent::Eof)) => {
                    break;
                }
                Ok(Err(ReadlineError::Closed)) => {}
                Ok(Err(ReadlineError::IO(e))) => {
                    println!("IO Error: {:?}", e);
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
        let _ = readline.flush();

        // Drop the stopper if we just broke out
        let _ = self.inner.lock().done.take();

        connection_state_jh.await;
    }
}

impl UI for InteractiveUI {
    fn set_command_processor(&mut self, cmdproc: CommandProcessor) {
        let mut inner = self.inner.lock();
        inner.cmdproc = Some(cmdproc);
    }
    fn run_async(&mut self) -> Pin<Box<dyn core::future::Future<Output = ()>>> {
        let this = self.clone();
        Box::pin(async move {
            this.command_loop().await;
        })
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct InteractiveUISender {
    inner: Arc<Mutex<InteractiveUIInner>>,
    connection_state_sender: flume::Sender<ConnectionState>,
}

impl UISender for InteractiveUISender {
    fn clone_uisender(&self) -> Box<dyn UISender> {
        Box::new(InteractiveUISender {
            inner: self.inner.clone(),
            connection_state_sender: self.connection_state_sender.clone(),
        })
    }
    fn as_logwriter(&self) -> Option<Box<dyn LogWriter>> {
        None
    }

    fn display_string_dialog(&self, title: &str, text: &str, close_cb: UICallback) {
        let Some(mut stdout) = self.inner.lock().stdout.clone() else {
            return;
        };
        if let Err(e) = writeln!(stdout, "{}: {}", title, text) {
            self.inner.lock().error = Some(e.to_string());
        }
        if let UICallback::Interactive(mut close_cb) = close_cb {
            close_cb()
        }
    }

    fn quit(&self) {
        self.inner.lock().done.take();
    }

    fn send_callback(&self, callback: UICallback) {
        if let UICallback::Interactive(mut callback) = callback {
            callback();
        }
    }
    fn set_attachment_state(
        &mut self,
        _state: &str,
        _public_internet_ready: bool,
        _local_network_ready: bool,
    ) {
        //
    }
    fn set_network_status(
        &mut self,
        _started: bool,
        _bps_down: u64,
        _bps_up: u64,
        mut _peers: Vec<json::JsonValue>,
    ) {
        //
    }
    fn set_config(&mut self, _config: &json::JsonValue) {
        //
    }
    fn set_connection_state(&mut self, state: ConnectionState) {
        if let Err(e) = self.connection_state_sender.send(state) {
            eprintln!("Error: {:?}", e);
            self.inner.lock().done.take();
        }
    }

    fn add_node_event(&self, _log_color: Level, event: &str) {
        let Some(mut stdout) = self.inner.lock().stdout.clone() else {
            return;
        };
        if let Err(e) = writeln!(stdout, "{}", event) {
            self.inner.lock().error = Some(e.to_string());
        }
    }
    fn add_log_event(&self, log_color: Level, event: &str) {
        let (enable_color, mut stdout) = {
            let inner = self.inner.lock();
            if !inner.log_enabled {
                return;
            }
            let Some(stdout) = inner.stdout.clone() else {
                return;
            };

            (inner.enable_color, stdout)
        };

        let log_line = format!(
            "{}: {}",
            CursiveUI::cli_ts(CursiveUI::get_start_time()),
            event
        );
        if enable_color {
            let log_line = match log_color {
                Level::Error => style(log_line).red().bright().to_string(),
                Level::Warn => style(log_line).yellow().bright().to_string(),
                Level::Info => log_line,
                Level::Debug => style(log_line).green().bright().to_string(),
                Level::Trace => style(log_line).blue().bright().to_string(),
            };
            if let Err(e) = writeln!(stdout, "{}", log_line) {
                eprintln!("Error: {:?}", e);
                self.inner.lock().done.take();
            }
        } else {
            println!("{}", log_line);
        }
    }
}

use crate::command_processor::*;
use crate::cursive_ui::CursiveUI;
use crate::settings::*;
use crate::tools::*;
use crate::ui::*;

use console::{style, Term};
use flexi_logger::writers::LogWriter;
use stop_token::future::FutureExt as StopTokenFutureExt;
use stop_token::*;

pub type LogViewerUICallback = Box<dyn FnMut() + Send>;

pub struct LogViewerUIInner {
    cmdproc: Option<CommandProcessor>,
    done: Option<StopSource>,
    term: Term,
    connection_state_receiver: flume::Receiver<ConnectionState>,
}

#[derive(Clone)]
pub struct LogViewerUI {
    inner: Arc<Mutex<LogViewerUIInner>>,
}

impl LogViewerUI {
    pub fn new(_settings: &Settings) -> (Self, LogViewerUISender) {
        let (cssender, csreceiver) = flume::unbounded::<ConnectionState>();

        let term = Term::stdout();
        let enable_color = console::colors_enabled() && term.features().colors_supported();

        // Create the UI object
        let this = Self {
            inner: Arc::new(Mutex::new(LogViewerUIInner {
                cmdproc: None,
                done: Some(StopSource::new()),
                term: term.clone(),
                connection_state_receiver: csreceiver,
            })),
        };

        let ui_sender = LogViewerUISender {
            inner: this.inner.clone(),
            connection_state_sender: cssender,
            term,
            enable_color,
        };

        (this, ui_sender)
    }

    pub async fn command_loop(&self) {
        let (connection_state_receiver, term, done) = {
            let inner = self.inner.lock();
            (
                inner.connection_state_receiver.clone(),
                inner.term.clone(),
                inner.done.as_ref().unwrap().token(),
            )
        };

        CursiveUI::set_start_time();

        // Wait for connection to be established
        loop {
            match connection_state_receiver.recv_async().await {
                Ok(ConnectionState::ConnectedTCP(_, _))
                | Ok(ConnectionState::ConnectedIPC(_, _)) => {
                    break;
                }
                Ok(ConnectionState::RetryingTCP(_, _)) | Ok(ConnectionState::RetryingIPC(_, _)) => {
                }
                Ok(ConnectionState::Disconnected) => {}
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    self.inner.lock().done.take();
                    break;
                }
            }
        }

        let cmdproc = self.inner.lock().cmdproc.clone().unwrap();

        if !term.features().is_attended() {
            done.await;
        } else {
            while let Ok(Ok(c)) = blocking_wrapper(
                {
                    let term = term.clone();
                    move || term.read_char()
                },
                Err(std::io::Error::other("failed")),
            )
            .timeout_at(done.clone())
            .await
            {
                match c {
                    'q' | 'Q' => {
                        self.inner.lock().done.take();
                        break;
                    }
                    'e' | 'E' => {
                        if let Err(e) = cmdproc.run_command(
                            "change_log_level api error",
                            UICallback::LogViewerUI(Box::new(|| {})),
                        ) {
                            eprintln!("Error: {:?}", e);
                            self.inner.lock().done.take();
                        }
                    }
                    'w' | 'W' => {
                        if let Err(e) = cmdproc.run_command(
                            "change_log_level api warn",
                            UICallback::LogViewerUI(Box::new(|| {})),
                        ) {
                            eprintln!("Error: {:?}", e);
                            self.inner.lock().done.take();
                        }
                    }
                    'i' | 'I' => {
                        if let Err(e) = cmdproc.run_command(
                            "change_log_level api info",
                            UICallback::LogViewerUI(Box::new(|| {})),
                        ) {
                            eprintln!("Error: {:?}", e);
                            self.inner.lock().done.take();
                        }
                    }
                    'd' | 'D' => {
                        if let Err(e) = cmdproc.run_command(
                            "change_log_level api debug",
                            UICallback::LogViewerUI(Box::new(|| {})),
                        ) {
                            eprintln!("Error: {:?}", e);
                            self.inner.lock().done.take();
                        }
                    }
                    't' | 'T' => {
                        if let Err(e) = cmdproc.run_command(
                            "change_log_level api trace",
                            UICallback::LogViewerUI(Box::new(|| {})),
                        ) {
                            eprintln!("Error: {:?}", e);
                            self.inner.lock().done.take();
                        }
                    }
                    'h' | 'H' => {
                        println!(
                            r"Help:
    h - This help
    e - Change log level to 'error'
    w - Change log level to 'warn'
    i - Change log level to 'info'
    d - Change log level to 'debug'
    t - Change log level to 'trace'
    q - Quit
"
                        );
                    }
                    _ => {
                        // ignore
                    }
                }
            }
        }
    }
}

impl UI for LogViewerUI {
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
pub struct LogViewerUISender {
    inner: Arc<Mutex<LogViewerUIInner>>,
    connection_state_sender: flume::Sender<ConnectionState>,
    term: Term,
    enable_color: bool,
}

impl UISender for LogViewerUISender {
    fn clone_uisender(&self) -> Box<dyn UISender> {
        Box::new(LogViewerUISender {
            inner: self.inner.clone(),
            connection_state_sender: self.connection_state_sender.clone(),
            term: self.term.clone(),
            enable_color: self.enable_color,
        })
    }
    fn as_logwriter(&self) -> Option<Box<dyn LogWriter>> {
        None
    }

    fn display_string_dialog(&self, title: &str, text: &str, close_cb: UICallback) {
        println!("{}: {}", title, text);
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
        println!("{}", event);
    }
    fn add_log_event(&self, log_color: Level, event: &str) {
        let log_line = format!(
            "{}: {}",
            CursiveUI::cli_ts(CursiveUI::get_start_time()),
            event
        );
        if self.enable_color {
            let log_line = match log_color {
                Level::Error => style(log_line).red().bright().to_string(),
                Level::Warn => style(log_line).yellow().bright().to_string(),
                Level::Info => log_line,
                Level::Debug => style(log_line).green().bright().to_string(),
                Level::Trace => style(log_line).blue().bright().to_string(),
            };
            if let Err(e) = self.term.write_line(&log_line) {
                eprintln!("Error: {:?}", e);
                self.inner.lock().done.take();
            }
        } else {
            println!("{}", log_line);
        }
    }
}

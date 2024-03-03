use std::io::Write;

use crate::command_processor::*;
use crate::settings::*;
use crate::tools::*;
use crate::ui::*;

use flexi_logger::writers::LogWriter;
use rustyline_async::SharedWriter;
use rustyline_async::{Readline, ReadlineError, ReadlineEvent};

pub type InteractiveUICallback = Box<dyn FnMut() + Send>;

pub struct InteractiveUIInner {
    cmdproc: Option<CommandProcessor>,
    stdout: Option<SharedWriter>,
    error: Option<String>,
    done: bool,
}

#[derive(Clone)]
pub struct InteractiveUI {
    inner: Arc<Mutex<InteractiveUIInner>>,
}

impl InteractiveUI {
    pub fn new(_settings: &Settings) -> (Self, InteractiveUISender) {
        // Create the UI object
        let this = Self {
            inner: Arc::new(Mutex::new(InteractiveUIInner {
                cmdproc: None,
                stdout: None,
                error: None,
                done: false,
            })),
        };

        let ui_sender = InteractiveUISender {
            inner: this.inner.clone(),
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

        self.inner.lock().stdout = Some(stdout.clone());

        loop {
            if self.inner.lock().done {
                break;
            }
            if let Some(e) = self.inner.lock().error.clone() {
                println!("Error: {:?}", e);
                break;
            }
            match readline.readline().await {
                Ok(ReadlineEvent::Line(line)) => {
                    let line = line.trim();
                    if line == "clear" {
                        if let Err(e) = readline.clear() {
                            println!("Error: {:?}", e);
                        }
                    } else if !line.is_empty() {
                        readline.add_history_entry(line.to_string());
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
                Ok(ReadlineEvent::Interrupted) => {
                    break;
                }
                Ok(ReadlineEvent::Eof) => {
                    break;
                }
                Err(ReadlineError::Closed) => {}
                Err(ReadlineError::IO(e)) => {
                    println!("IO Error: {:?}", e);
                    break;
                }
            }
        }
        let _ = readline.flush();
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
}

impl UISender for InteractiveUISender {
    fn clone_uisender(&self) -> Box<dyn UISender> {
        Box::new(self.clone())
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
        self.inner.lock().done = true;
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
    fn set_connection_state(&mut self, _state: ConnectionState) {
        //
    }

    fn add_node_event(&self, _log_color: Level, event: &str) {
        let Some(mut stdout) = self.inner.lock().stdout.clone() else {
            return;
        };
        if let Err(e) = writeln!(stdout, "{}", event) {
            self.inner.lock().error = Some(e.to_string());
        }
    }
    fn add_log_event(&self, _log_color: Level, _event: &str) {}
}

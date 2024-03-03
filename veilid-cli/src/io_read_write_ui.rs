use crate::command_processor::*;
use crate::settings::*;
use crate::tools::*;
use crate::ui::*;

use futures::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use stop_token::future::FutureExt as StopTokenFutureExt;
use stop_token::*;
use veilid_tools::AsyncMutex;

use flexi_logger::writers::LogWriter;

static FINISHED_LINE: &str = "\x7F ===FINISHED=== \x7F";

pub type IOReadWriteUICallback = Box<dyn FnMut() + Send>;

pub struct IOReadWriteUIInner<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> {
    cmdproc: Option<CommandProcessor>,
    in_io: Arc<AsyncMutex<BufReader<R>>>,
    out_io: Arc<AsyncMutex<BufWriter<W>>>,
    out_receiver: flume::Receiver<String>,
    out_sender: flume::Sender<String>,
    done: Option<StopSource>,
    connection_state_receiver: flume::Receiver<ConnectionState>,
}

pub struct IOReadWriteUI<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> {
    inner: Arc<Mutex<IOReadWriteUIInner<R, W>>>,
}
impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> Clone for IOReadWriteUI<R, W> {
    fn clone(&self) -> Self {
        IOReadWriteUI {
            inner: self.inner.clone(),
        }
    }
}

impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> IOReadWriteUI<R, W> {
    pub fn new(_settings: &Settings, in_io: R, out_io: W) -> (Self, IOReadWriteUISender<R, W>) {
        // Create the UI object
        let (sender, receiver) = flume::unbounded::<String>();
        let (cssender, csreceiver) = flume::unbounded::<ConnectionState>();
        let this = Self {
            inner: Arc::new(Mutex::new(IOReadWriteUIInner {
                cmdproc: None,
                in_io: Arc::new(AsyncMutex::new(BufReader::new(in_io))),
                out_io: Arc::new(AsyncMutex::new(BufWriter::new(out_io))),
                out_receiver: receiver,
                out_sender: sender.clone(),
                connection_state_receiver: csreceiver,
                done: Some(StopSource::new()),
            })),
        };

        let ui_sender = IOReadWriteUISender {
            inner: this.inner.clone(),
            out_sender: sender,
            connection_state_sender: cssender,
        };

        (this, ui_sender)
    }

    pub async fn output_loop(&self) {
        let out_receiver = self.inner.lock().out_receiver.clone();
        let out_io = self.inner.lock().out_io.clone();

        let mut out = out_io.lock().await;
        let done = self.inner.lock().done.as_ref().unwrap().token();

        while let Ok(Ok(line)) = out_receiver.recv_async().timeout_at(done.clone()).await {
            if line == FINISHED_LINE {
                break;
            }
            let line = format!("{}\n", line);
            if let Err(e) = out.write_all(line.as_bytes()).await {
                eprintln!("Error: {:?}", e);
                break;
            }
            if let Err(e) = out.flush().await {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    pub async fn command_loop(&self) {
        let (in_io, out_sender, connection_state_receiver) = {
            let inner = self.inner.lock();
            (
                inner.in_io.clone(),
                inner.out_sender.clone(),
                inner.connection_state_receiver.clone(),
            )
        };
        let mut in_io = in_io.lock().await;

        let done = self.inner.lock().done.as_ref().unwrap().token();
        let (exec_sender, exec_receiver) = flume::bounded(1);

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

        // Process the input
        loop {
            let mut line = String::new();
            match in_io.read_line(&mut line).timeout_at(done.clone()).await {
                Ok(Ok(bytes)) => {
                    if bytes == 0 {
                        // Clean exit after everything else is sent
                        if let Err(e) = out_sender.send(FINISHED_LINE.to_string()) {
                            eprintln!("Error: {:?}", e);
                            self.inner.lock().done.take();
                        }
                        break;
                    }
                    let line = line.trim();
                    if !line.is_empty() {
                        let cmdproc = self.inner.lock().cmdproc.clone();
                        if let Some(cmdproc) = &cmdproc {
                            // Run command
                            if let Err(e) = cmdproc.run_command(
                                line,
                                UICallback::IOReadWrite(Box::new({
                                    let exec_sender = exec_sender.clone();
                                    move || {
                                        // Let the next command execute
                                        if let Err(e) = exec_sender.send(()) {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    }
                                })),
                            ) {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                                break;
                            }
                            // Wait until command is done executing before running the next line
                            if let Err(e) = exec_receiver.recv_async().await {
                                eprintln!("Error: {:?}", e);
                                self.inner.lock().done.take();
                                break;
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("IO Error: {:?}", e);
                    self.inner.lock().done.take();
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

impl<R: AsyncRead + Unpin + Send + 'static, W: AsyncWrite + Unpin + Send + 'static> UI
    for IOReadWriteUI<R, W>
{
    fn set_command_processor(&mut self, cmdproc: CommandProcessor) {
        let mut inner = self.inner.lock();
        inner.cmdproc = Some(cmdproc);
    }
    fn run_async(&mut self) -> Pin<Box<dyn core::future::Future<Output = ()>>> {
        let this = self.clone();
        Box::pin(async move {
            let out_fut = this.output_loop();
            let cmd_fut = this.command_loop();
            futures::join!(out_fut, cmd_fut);
        })
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct IOReadWriteUISender<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> {
    inner: Arc<Mutex<IOReadWriteUIInner<R, W>>>,
    out_sender: flume::Sender<String>,
    connection_state_sender: flume::Sender<ConnectionState>,
}

impl<R: AsyncRead + Unpin + Send + 'static, W: AsyncWrite + Unpin + Send + 'static> UISender
    for IOReadWriteUISender<R, W>
{
    fn clone_uisender(&self) -> Box<dyn UISender> {
        Box::new(IOReadWriteUISender {
            inner: self.inner.clone(),
            out_sender: self.out_sender.clone(),
            connection_state_sender: self.connection_state_sender.clone(),
        })
    }
    fn as_logwriter(&self) -> Option<Box<dyn LogWriter>> {
        None
    }

    fn display_string_dialog(&self, title: &str, text: &str, close_cb: UICallback) {
        if let Err(e) = self.out_sender.send(format!("{}: {}", title, text)) {
            eprintln!("Error: {:?}", e);
            self.inner.lock().done.take();
        }
        if let UICallback::IOReadWrite(mut close_cb) = close_cb {
            close_cb()
        }
    }

    fn quit(&self) {
        self.inner.lock().done.take();
    }

    fn send_callback(&self, callback: UICallback) {
        if let UICallback::IOReadWrite(mut callback) = callback {
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
        if let Err(e) = self.out_sender.send(format!("{}\n", event)) {
            eprintln!("Error: {:?}", e);
            self.inner.lock().done.take();
        }
    }
    fn add_log_event(&self, _log_color: Level, _event: &str) {}
}

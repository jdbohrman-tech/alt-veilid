use crate::client_api_connection::*;
use crate::settings::Settings;
use crate::ui::*;
use std::cell::*;
use std::net::SocketAddr;
use std::rc::Rc;
use std::time::SystemTime;
use veilid_core::tools::*;
use veilid_core::*;

pub fn convert_loglevel(s: &str) -> Result<VeilidConfigLogLevel, String> {
    match s.to_ascii_lowercase().as_str() {
        "off" => Ok(VeilidConfigLogLevel::Off),
        "error" => Ok(VeilidConfigLogLevel::Error),
        "warn" => Ok(VeilidConfigLogLevel::Warn),
        "info" => Ok(VeilidConfigLogLevel::Info),
        "debug" => Ok(VeilidConfigLogLevel::Debug),
        "trace" => Ok(VeilidConfigLogLevel::Trace),
        _ => Err(format!("Invalid log level: {}", s)),
    }
}

#[derive(PartialEq, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connected(SocketAddr, SystemTime),
    Retrying(SocketAddr, SystemTime),
}
impl ConnectionState {
    pub fn is_disconnected(&self) -> bool {
        matches!(*self, Self::Disconnected)
    }
    pub fn is_connected(&self) -> bool {
        matches!(*self, Self::Connected(_, _))
    }
    pub fn is_retrying(&self) -> bool {
        matches!(*self, Self::Retrying(_, _))
    }
}

struct CommandProcessorInner {
    ui: UI,
    capi: Option<ClientApiConnection>,
    reconnect: bool,
    finished: bool,
    autoconnect: bool,
    autoreconnect: bool,
    server_addr: Option<SocketAddr>,
    connection_waker: Eventual,
    last_call_id: Option<OperationId>,
}

type Handle<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct CommandProcessor {
    inner: Handle<CommandProcessorInner>,
}

impl CommandProcessor {
    pub fn new(ui: UI, settings: &Settings) -> Self {
        Self {
            inner: Rc::new(RefCell::new(CommandProcessorInner {
                ui,
                capi: None,
                reconnect: settings.autoreconnect,
                finished: false,
                autoconnect: settings.autoconnect,
                autoreconnect: settings.autoreconnect,
                server_addr: None,
                connection_waker: Eventual::new(),
                last_call_id: None,
            })),
        }
    }
    pub fn set_client_api_connection(&mut self, capi: ClientApiConnection) {
        self.inner.borrow_mut().capi = Some(capi);
    }
    fn inner(&self) -> Ref<CommandProcessorInner> {
        self.inner.borrow()
    }
    fn inner_mut(&self) -> RefMut<CommandProcessorInner> {
        self.inner.borrow_mut()
    }
    fn ui(&self) -> UI {
        self.inner.borrow().ui.clone()
    }
    fn capi(&self) -> ClientApiConnection {
        self.inner.borrow().capi.as_ref().unwrap().clone()
    }

    fn word_split(line: &str) -> (String, Option<String>) {
        let trimmed = line.trim();
        if let Some(p) = trimmed.find(char::is_whitespace) {
            let first = trimmed[0..p].to_owned();
            let rest = trimmed[p..].trim_start().to_owned();
            (first, Some(rest))
        } else {
            (trimmed.to_owned(), None)
        }
    }

    pub fn cancel_command(&self) {
        trace!("CommandProcessor::cancel_command");
        let capi = self.capi();
        capi.cancel();
    }

    pub fn cmd_help(&self, _rest: Option<String>, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_help");
        self.ui().add_node_event(
            r#"Commands:
exit/quit           - exit the client
disconnect          - disconnect the client from the Veilid node 
shutdown            - shut the server down
attach              - attach the server to the Veilid network
detach              - detach the server from the Veilid network
debug               - send a debugging command to the Veilid server
change_log_level    - change the log level for a tracing layer
reply               - reply to an AppCall not handled directly by the server
"#
            .to_owned(),
        );
        let ui = self.ui();
        ui.send_callback(callback);
        Ok(())
    }

    pub fn cmd_exit(&self, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_exit");
        let ui = self.ui();
        ui.send_callback(callback);
        ui.quit();
        Ok(())
    }

    pub fn cmd_shutdown(&self, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_shutdown");
        let mut capi = self.capi();
        let ui = self.ui();
        spawn_detached_local(async move {
            if let Err(e) = capi.server_shutdown().await {
                error!("Server command 'shutdown' failed to execute: {}", e);
            }
            ui.send_callback(callback);
        });
        Ok(())
    }

    pub fn cmd_attach(&self, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_attach");
        let mut capi = self.capi();
        let ui = self.ui();
        spawn_detached_local(async move {
            if let Err(e) = capi.server_attach().await {
                error!("Server command 'attach' failed: {}", e);
            }
            ui.send_callback(callback);
        });
        Ok(())
    }

    pub fn cmd_detach(&self, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_detach");
        let mut capi = self.capi();
        let ui = self.ui();
        spawn_detached_local(async move {
            if let Err(e) = capi.server_detach().await {
                error!("Server command 'detach' failed: {}", e);
            }
            ui.send_callback(callback);
        });
        Ok(())
    }

    pub fn cmd_disconnect(&self, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_disconnect");
        let mut capi = self.capi();
        let ui = self.ui();
        spawn_detached_local(async move {
            capi.disconnect().await;
            ui.send_callback(callback);
        });
        Ok(())
    }

    pub fn cmd_debug(&self, rest: Option<String>, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_debug");
        let mut capi = self.capi();
        let ui = self.ui();
        spawn_detached_local(async move {
            match capi.server_debug(rest.unwrap_or_default()).await {
                Ok(output) => ui.display_string_dialog("Debug Output", output, callback),
                Err(e) => ui.display_string_dialog("Debug Error", e.to_string(), callback),
            }
        });
        Ok(())
    }

    pub fn cmd_change_log_level(
        &self,
        rest: Option<String>,
        callback: UICallback,
    ) -> Result<(), String> {
        trace!("CommandProcessor::cmd_change_log_level");
        let mut capi = self.capi();
        let ui = self.ui();
        spawn_detached_local(async move {
            let (layer, rest) = Self::word_split(&rest.unwrap_or_default());
            let log_level = match convert_loglevel(&rest.unwrap_or_default()) {
                Ok(v) => v,
                Err(e) => {
                    ui.add_node_event(format!("Failed to change log level: {}", e));
                    ui.send_callback(callback);
                    return;
                }
            };

            match capi.server_change_log_level(layer, log_level).await {
                Ok(()) => {
                    ui.display_string_dialog("Success", "Log level changed", callback);
                }
                Err(e) => {
                    ui.display_string_dialog(
                        "Server command 'change_log_level' failed",
                        e.to_string(),
                        callback,
                    );
                }
            }
        });
        Ok(())
    }

    pub fn cmd_reply(&self, rest: Option<String>, callback: UICallback) -> Result<(), String> {
        trace!("CommandProcessor::cmd_reply");

        let mut capi = self.capi();
        let ui = self.ui();
        let some_last_id = self.inner_mut().last_call_id.take();
        spawn_detached_local(async move {
            let (first, second) = Self::word_split(&rest.clone().unwrap_or_default());
            let (id, msg) = if let Some(second) = second {
                let id = match u64::from_str(&first) {
                    Err(e) => {
                        ui.add_node_event(format!("invalid appcall id: {}", e));
                        ui.send_callback(callback);
                        return;
                    }
                    Ok(v) => v,
                };
                (OperationId::new(id), second)
            } else {
                let id = match some_last_id {
                    None => {
                        ui.add_node_event("must specify last call id".to_owned());
                        ui.send_callback(callback);
                        return;
                    }
                    Some(v) => v,
                };
                (id, rest.unwrap_or_default())
            };
            let msg = if msg[0..1] == "#".to_owned() {
                match hex::decode(msg[1..].as_bytes().to_vec()) {
                    Err(e) => {
                        ui.add_node_event(format!("invalid hex message: {}", e));
                        ui.send_callback(callback);
                        return;
                    }
                    Ok(v) => v,
                }
            } else {
                msg[1..].as_bytes().to_vec()
            };
            let msglen = msg.len();
            match capi.server_appcall_reply(id, msg).await {
                Ok(()) => {
                    ui.add_node_event(format!("reply sent to {} : {} bytes", id, msglen));
                    ui.send_callback(callback);
                    return;
                }
                Err(e) => {
                    ui.display_string_dialog(
                        "Server command 'appcall_reply' failed",
                        e.to_string(),
                        callback,
                    );
                }
            }
        });
        Ok(())
    }

    pub fn run_command(&self, command_line: &str, callback: UICallback) -> Result<(), String> {
        //
        let (cmd, rest) = Self::word_split(command_line);
        match cmd.as_str() {
            "help" => self.cmd_help(rest, callback),
            "exit" => self.cmd_exit(callback),
            "quit" => self.cmd_exit(callback),
            "disconnect" => self.cmd_disconnect(callback),
            "shutdown" => self.cmd_shutdown(callback),
            "attach" => self.cmd_attach(callback),
            "detach" => self.cmd_detach(callback),
            "debug" => self.cmd_debug(rest, callback),
            "change_log_level" => self.cmd_change_log_level(rest, callback),
            "reply" => self.cmd_reply(rest, callback),
            _ => {
                let ui = self.ui();
                ui.send_callback(callback);
                Err(format!("Invalid command: {}", cmd))
            }
        }
    }

    pub async fn connection_manager(&mut self) {
        // Connect until we're done
        while !self.inner_mut().finished {
            // Wait for connection request
            if !self.inner().autoconnect {
                let waker = self.inner_mut().connection_waker.instance_clone(());
                waker.await;
            } else {
                self.inner_mut().autoconnect = false;
            }
            self.inner_mut().connection_waker.reset();
            // Loop while we want to keep the connection
            let mut first = true;
            while self.inner().reconnect {
                let server_addr_opt = self.inner_mut().server_addr;
                let server_addr = match server_addr_opt {
                    None => break,
                    Some(addr) => addr,
                };
                if first {
                    info!("Connecting to server at {}", server_addr);
                    self.set_connection_state(ConnectionState::Retrying(
                        server_addr,
                        SystemTime::now(),
                    ));
                } else {
                    debug!("Retrying connection to {}", server_addr);
                }
                let mut capi = self.capi();
                let res = capi.connect(server_addr).await;
                if res.is_ok() {
                    info!(
                        "Connection to server at {} terminated normally",
                        server_addr
                    );
                    break;
                }
                if !self.inner().autoreconnect {
                    info!("Connection to server lost.");
                    break;
                }

                self.set_connection_state(ConnectionState::Retrying(
                    server_addr,
                    SystemTime::now(),
                ));

                debug!("Connection lost, retrying in 2 seconds");
                {
                    let waker = self.inner_mut().connection_waker.instance_clone(());
                    let _ = timeout(2000, waker).await;
                }
                self.inner_mut().connection_waker.reset();
                first = false;
            }
            info!("Disconnected.");
            self.set_connection_state(ConnectionState::Disconnected);
            self.inner_mut().reconnect = true;
        }
    }

    // called by ui
    ////////////////////////////////////////////
    pub fn set_server_address(&mut self, server_addr: Option<SocketAddr>) {
        self.inner_mut().server_addr = server_addr;
    }
    pub fn get_server_address(&self) -> Option<SocketAddr> {
        self.inner().server_addr
    }
    // called by client_api_connection
    // calls into ui
    ////////////////////////////////////////////

    pub fn update_attachment(&mut self, attachment: veilid_core::VeilidStateAttachment) {
        self.inner_mut().ui.set_attachment_state(
            attachment.state,
            attachment.public_internet_ready,
            attachment.local_network_ready,
        );
    }

    pub fn update_network_status(&mut self, network: veilid_core::VeilidStateNetwork) {
        self.inner_mut().ui.set_network_status(
            network.started,
            network.bps_down.as_u64(),
            network.bps_up.as_u64(),
            network.peers,
        );
    }
    pub fn update_config(&mut self, config: veilid_core::VeilidStateConfig) {
        self.inner_mut().ui.set_config(config.config)
    }
    pub fn update_route(&mut self, route: veilid_core::VeilidStateRoute) {
        let mut out = String::new();
        if !route.dead_routes.is_empty() {
            out.push_str(&format!("Dead routes: {:?}", route.dead_routes));
        }
        if !route.dead_remote_routes.is_empty() {
            if !out.is_empty() {
                out.push_str("\n");
            }
            out.push_str(&format!(
                "Dead remote routes: {:?}",
                route.dead_remote_routes
            ));
        }
        if !out.is_empty() {
            self.inner().ui.add_node_event(out);
        }
    }
    pub fn update_value_change(&mut self, value_change: veilid_core::VeilidValueChange) {
        let out = format!("Value change: {:?}", value_change);
        self.inner().ui.add_node_event(out);
    }

    pub fn update_log(&mut self, log: veilid_core::VeilidLog) {
        self.inner().ui.add_node_event(format!(
            "{}: {}{}",
            log.log_level,
            log.message,
            if let Some(bt) = log.backtrace {
                format!("\nBacktrace:\n{}", bt)
            } else {
                "".to_owned()
            }
        ));
    }

    pub fn update_app_message(&mut self, msg: veilid_core::VeilidAppMessage) {
        // check is message body is ascii printable
        let mut printable = true;
        for c in msg.message() {
            if *c < 32 || *c > 126 {
                printable = false;
            }
        }

        let strmsg = if printable {
            String::from_utf8_lossy(msg.message()).to_string()
        } else {
            hex::encode(msg.message())
        };

        self.inner()
            .ui
            .add_node_event(format!("AppMessage ({:?}): {}", msg.sender(), strmsg));
    }

    pub fn update_app_call(&mut self, call: veilid_core::VeilidAppCall) {
        // check is message body is ascii printable
        let mut printable = true;
        for c in call.message() {
            if *c < 32 || *c > 126 {
                printable = false;
            }
        }

        let strmsg = if printable {
            String::from_utf8_lossy(call.message()).to_string()
        } else {
            format!("#{}", hex::encode(call.message()))
        };

        self.inner().ui.add_node_event(format!(
            "AppCall ({:?}) id = {:016x} : {}",
            call.sender(),
            call.id().as_u64(),
            strmsg
        ));

        self.inner_mut().last_call_id = Some(call.id());
    }

    pub fn update_shutdown(&mut self) {
        // Do nothing with this, we'll process shutdown when rpc connection closes
    }

    // called by client_api_connection
    // calls into ui
    ////////////////////////////////////////////
    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.inner_mut().ui.set_connection_state(state);
    }
    // called by ui
    ////////////////////////////////////////////
    pub fn start_connection(&mut self) {
        self.inner_mut().reconnect = true;
        self.inner_mut().connection_waker.resolve();
    }
    // pub fn stop_connection(&mut self) {
    //     self.inner_mut().reconnect = false;
    //     let mut capi = self.capi().clone();
    //     spawn_detached(async move {
    //         capi.disconnect().await;
    //     });
    // }
    pub fn cancel_reconnect(&mut self) {
        self.inner_mut().reconnect = false;
        self.inner_mut().connection_waker.resolve();
    }
    pub fn quit(&mut self) {
        self.inner_mut().finished = true;
        self.inner_mut().reconnect = false;
        self.inner_mut().connection_waker.resolve();
    }

    // called by ui
    // calls into client_api_connection
    ////////////////////////////////////////////
    pub fn attach(&mut self) {
        let mut capi = self.capi();

        spawn_detached_local(async move {
            if let Err(e) = capi.server_attach().await {
                error!("Server command 'attach' failed to execute: {}", e);
            }
        });
    }

    pub fn detach(&mut self) {
        let mut capi = self.capi();

        spawn_detached_local(async move {
            if let Err(e) = capi.server_detach().await {
                error!("Server command 'detach' failed to execute: {}", e);
            }
        });
    }
}

use crate::command_processor::*;
use crate::cursive_ui::CursiveUICallback;
use crate::interactive_ui::InteractiveUICallback;
use crate::io_read_write_ui::IOReadWriteUICallback;
use crate::log_viewer_ui::LogViewerUICallback;
use crate::tools::*;
use flexi_logger::writers::LogWriter;
use log::Level;

pub enum UICallback {
    Cursive(CursiveUICallback),
    Interactive(InteractiveUICallback),
    IOReadWrite(IOReadWriteUICallback),
    LogViewerUI(LogViewerUICallback),
}

pub trait UISender: Send {
    fn clone_uisender(&self) -> Box<dyn UISender>;
    fn as_logwriter(&self) -> Option<Box<dyn LogWriter>>;

    fn display_string_dialog(&self, title: &str, text: &str, close_cb: UICallback);
    fn quit(&self);
    fn send_callback(&self, callback: UICallback);
    fn set_attachment_state(
        &mut self,
        state: &str,
        public_internet_ready: bool,
        local_network_ready: bool,
    );
    fn set_network_status(
        &mut self,
        started: bool,
        bps_down: u64,
        bps_up: u64,
        peers: Vec<json::JsonValue>,
    );
    fn set_config(&mut self, config: &json::JsonValue);
    fn set_connection_state(&mut self, state: ConnectionState);
    fn add_node_event(&self, log_color: Level, event: &str);
    fn add_log_event(&self, log_color: Level, event: &str);
}

pub trait UI {
    fn set_command_processor(&mut self, cmdproc: CommandProcessor);
    fn run_async<'a>(&'a mut self) -> Pin<Box<dyn core::future::Future<Output = ()> + 'a>>;
}

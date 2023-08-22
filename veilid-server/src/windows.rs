use crate::settings::*;
use crate::*;
use std::ffi::OsString;
use std::time::Duration;
use tracing::*;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::ServiceControlHandlerResult;
use windows_service::*;

// Register generated `ffi_service_main` with the system and start the service, blocking
// this thread until the service is stopped.
pub fn run_service(settings: Settings, _args: CmdlineArgs) -> EyreResult<()> {
    eprintln!("Windows Service mode not implemented yet.");

    //service_dispatcher::start("veilid-server", ffi_veilid_service_main)?;
    //
    Ok(())
}

///////////////
define_windows_service!(ffi_veilid_service_main, veilid_service_main);
fn veilid_service_main(arguments: Vec<OsString>) {
    if let Err(e) = register_service_handler(arguments) {
        error!("{}", e);
    }
}

///////////////

fn register_service_handler(arguments: Vec<OsString>) -> windows_service::Result<()> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                // Handle stop event and return control back to the system.
                ServiceControlHandlerResult::NoError
            }
            // All services must accept Interrogate even if it's a no-op.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler
    let status_handle = service_control_handler::register("veilid-server", event_handler)?;

    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(),
        // Unused for setting status
        process_id: None,
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status)?;

    // Do some work

    Ok(())
}

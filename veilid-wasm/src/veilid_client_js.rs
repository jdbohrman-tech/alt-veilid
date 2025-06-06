#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen(typescript_custom_section)]
const IUPDATE_VEILID_FUNCTION: &'static str = r#"
export type UpdateVeilidFunction = (event: VeilidUpdate) => void;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, typescript_type = "UpdateVeilidFunction")]
    pub type UpdateVeilidFunction;
}

#[wasm_bindgen(js_name = veilidClient)]
pub struct VeilidClient {}

// Since this implementation doesn't contain a `new` fn that's marked as a constructor,
// and none of the member fns take a &self arg,
// this is just a namespace/class of static functions.
#[wasm_bindgen(js_class = veilidClient)]
impl VeilidClient {
    #[allow(clippy::unused_async)]
    pub async fn initializeCore(platformConfig: VeilidWASMConfig) {
        if INITIALIZED.swap(true, Ordering::AcqRel) {
            return;
        }
        console_error_panic_hook::set_once();

        // Set up subscriber and layers
        let subscriber = Registry::default();
        let mut layers = Vec::new();
        let mut filters = (*FILTERS).borrow_mut();

        // Performance logger
        if platformConfig.logging.performance.enabled {
            let filter = veilid_core::VeilidLayerFilter::new(
                platformConfig.logging.performance.level,
                &platformConfig.logging.performance.ignore_log_targets,
                None,
            );
            let layer = WASMLayer::new(
                WASMLayerConfig::new()
                    .with_report_logs_in_timings(platformConfig.logging.performance.logs_in_timings)
                    .with_console_config(if platformConfig.logging.performance.logs_in_console {
                        ConsoleConfig::ReportWithConsoleColor
                    } else {
                        ConsoleConfig::NoReporting
                    })
                    .with_field_filter(Some(Arc::new(|k| k != veilid_core::VEILID_LOG_KEY_FIELD))),
            )
            .with_filter(filter.clone());
            filters.insert("performance", filter);
            layers.push(layer.boxed());
        };

        // API logger
        if platformConfig.logging.api.enabled {
            let filter = veilid_core::VeilidLayerFilter::new(
                platformConfig.logging.api.level,
                &platformConfig.logging.api.ignore_log_targets,
                None,
            );
            let layer = veilid_core::ApiTracingLayer::init().with_filter(filter.clone());
            filters.insert("api", filter);
            layers.push(layer.boxed());
        }

        let subscriber = subscriber.with(layers);
        subscriber
            .try_init()
            .map_err(|e| format!("failed to initialize logging: {}", e))
            .expect("failed to initalize WASM platform");
    }

    /// Initialize a Veilid node, with the configuration in JSON format
    ///
    /// Must be called only once at the start of an application
    ///
    /// @param {UpdateVeilidFunction} update_callback_js - called when internal state of the Veilid node changes, for example, when app-level messages are received, when private routes die and need to be reallocated, or when routing table states change
    /// @param {string} json_config - called at startup to supply a JSON configuration object.
    pub async fn startupCore(
        update_callback_js: UpdateVeilidFunction,
        config: VeilidConfig,
    ) -> APIResult<()> {
        let update_callback_js = SendWrapper::new(update_callback_js);
        let update_callback = Arc::new(move |update: VeilidUpdate| {
            let _ret = match Function::call1(
                &update_callback_js,
                &JsValue::UNDEFINED,
                &to_jsvalue(update),
            ) {
                Ok(v) => v,
                Err(e) => {
                    console_log(&format!("calling update callback failed: {:?}", e));
                    return;
                }
            };
        });

        if VEILID_API.borrow().is_some() {
            return APIResult::Err(veilid_core::VeilidAPIError::AlreadyInitialized);
        }

        let veilid_api = veilid_core::api_startup_config(update_callback, config).await?;
        VEILID_API.replace(Some(veilid_api));
        APIRESULT_UNDEFINED
    }

    // TODO: can we refine the TS type of `layer`?
    pub fn changeLogLevel(layer: String, log_level: VeilidConfigLogLevel) {
        let layer = if layer == "all" { "".to_owned() } else { layer };
        let filters = (*FILTERS).borrow();
        if layer.is_empty() {
            // Change all layers
            for f in filters.values() {
                f.set_max_level(log_level);
            }
        } else {
            // Change a specific layer
            if let Some(f) = filters.get(layer.as_str()) {
                f.set_max_level(log_level);
            }
        }
    }

    // TODO: can we refine the TS type of `layer`?
    pub fn changeLogIgnore(layer: String, changes: Vec<String>) {
        let layer = if layer == "all" { "".to_owned() } else { layer };
        let filters = (*FILTERS).borrow();
        if layer.is_empty() {
            // Change all layers
            for f in filters.values() {
                let mut ignore_list = f.ignore_list();
                VeilidLayerFilter::apply_ignore_change_list(&mut ignore_list, &changes);
                f.set_ignore_list(Some(ignore_list));
            }
        } else {
            // Change a specific layer
            if let Some(f) = filters.get(layer.as_str()) {
                let mut ignore_list = f.ignore_list();
                VeilidLayerFilter::apply_ignore_change_list(&mut ignore_list, &changes);
                f.set_ignore_list(Some(ignore_list));
            }
        }
    }
    /// Shut down Veilid and terminate the API.
    pub async fn shutdownCore() -> APIResult<()> {
        let veilid_api = take_veilid_api()?;
        veilid_api.shutdown().await;
        APIRESULT_UNDEFINED
    }

    /// Check if Veilid is shutdown.
    pub fn isShutdown() -> APIResult<bool> {
        let veilid_api = get_veilid_api();
        if let Err(veilid_core::VeilidAPIError::NotInitialized) = veilid_api {
            return APIResult::Ok(true);
        }
        let veilid_api = veilid_api.unwrap();
        let is_shutdown = veilid_api.is_shutdown();
        APIResult::Ok(is_shutdown)
    }

    /// Get a full copy of the current state of Veilid.
    pub async fn getState() -> APIResult<VeilidState> {
        let veilid_api = get_veilid_api()?;
        let core_state = veilid_api.get_state().await?;
        APIResult::Ok(core_state)
    }

    /// Connect to the network.
    pub async fn attach() -> APIResult<()> {
        let veilid_api = get_veilid_api()?;
        veilid_api.attach().await?;
        APIRESULT_UNDEFINED
    }

    /// Disconnect from the network.
    pub async fn detach() -> APIResult<()> {
        let veilid_api = get_veilid_api()?;
        veilid_api.detach().await?;
        APIRESULT_UNDEFINED
    }

    /// Get the current timestamp, in string format
    #[must_use]
    pub fn now() -> String {
        veilid_core::Timestamp::now().as_u64().to_string()
    }

    /// Execute an 'internal debug command'.
    pub async fn debug(command: String) -> APIResult<String> {
        let veilid_api = get_veilid_api()?;
        let out = veilid_api.debug(command).await?;
        APIResult::Ok(out)
    }

    /// Return the cargo package version of veilid-core, in object format.
    #[must_use]
    pub fn version() -> VeilidVersion {
        let (major, minor, patch) = veilid_core::veilid_version();
        super::VeilidVersion {
            major,
            minor,
            patch,
        }
    }

    /// Return the features that were enabled when veilid-core was built.
    #[must_use]
    pub fn features() -> Vec<String> {
        veilid_core::veilid_features()
    }

    /// Return the cargo package version of veilid-core, in string format.
    #[must_use]
    pub fn versionString() -> String {
        veilid_core::veilid_version_string()
    }

    /// Return the default veilid configuration, in string format
    pub fn defaultConfig() -> VeilidConfig {
        VeilidConfig::default()
    }
}

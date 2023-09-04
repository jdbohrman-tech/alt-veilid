#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen(typescript_custom_section)]
const IUPDATE_VEILID_FUNCTION: &'static str = r#"
type UpdateVeilidFunction = (event: VeilidUpdate) => void;
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
    pub async fn initializeCore(platformConfig: VeilidWASMConfig) {
        if INITIALIZED.swap(true, Ordering::Relaxed) {
            return;
        }
        console_error_panic_hook::set_once();

        // Set up subscriber and layers
        let subscriber = Registry::default();
        let mut layers = Vec::new();
        let mut filters = (*FILTERS).borrow_mut();

        // Performance logger
        if platformConfig.logging.performance.enabled {
            let filter =
                veilid_core::VeilidLayerFilter::new(platformConfig.logging.performance.level, None);
            let layer = WASMLayer::new(
                WASMLayerConfigBuilder::new()
                    .set_report_logs_in_timings(platformConfig.logging.performance.logs_in_timings)
                    .set_console_config(if platformConfig.logging.performance.logs_in_console {
                        ConsoleConfig::ReportWithConsoleColor
                    } else {
                        ConsoleConfig::NoReporting
                    })
                    .build(),
            )
            .with_filter(filter.clone());
            filters.insert("performance", filter);
            layers.push(layer.boxed());
        };

        // API logger
        if platformConfig.logging.api.enabled {
            let filter =
                veilid_core::VeilidLayerFilter::new(platformConfig.logging.api.level, None);
            let layer = veilid_core::ApiTracingLayer::get().with_filter(filter.clone());
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
        json_config: String,
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

        let veilid_api = veilid_core::api_startup_json(update_callback, json_config).await?;
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
            let f = filters.get(layer.as_str()).unwrap();
            f.set_max_level(log_level);
        }
    }

    /// Shut down Veilid and terminate the API.
    pub async fn shutdownCore() -> APIResult<()> {
        let veilid_api = take_veilid_api()?;
        veilid_api.shutdown().await;
        APIRESULT_UNDEFINED
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

    /// Execute an 'internal debug command'.
    pub async fn debug(command: String) -> APIResult<String> {
        let veilid_api = get_veilid_api()?;
        let out = veilid_api.debug(command).await?;
        APIResult::Ok(out)
    }

    /// Return the cargo package version of veilid-core, in object format.
    pub fn version() -> VeilidVersion {
        let (major, minor, patch) = veilid_core::veilid_version();
        let vv = super::VeilidVersion {
            major,
            minor,
            patch,
        };
        vv
    }

    /// Return the cargo package version of veilid-core, in string format.
    pub fn versionString() -> String {
        veilid_core::veilid_version_string()
    }
}

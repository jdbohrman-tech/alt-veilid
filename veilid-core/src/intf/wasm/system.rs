use crate::*;

//use js_sys::*;

pub async fn get_outbound_relay_peer() -> Option<crate::routing_table::PeerInfo> {
    // unimplemented!
    None
}

// pub async fn get_pwa_web_server_config() -> {
//     if is_browser() {

//         let win = window().unwrap();
//         let doc = win.document().unwrap();
//         let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
//         let cookie = html_document.cookie().unwrap();

//         // let wait_millis = if millis > u32::MAX {
//         //     i32::MAX
//         // } else {
//         //     millis as i32
//         // };
//         // let promise = Promise::new(&mut |yes, _| {
//         //     let win = window().unwrap();
//         //     win.set_timeout_with_callback_and_timeout_and_arguments_0(&yes, wait_millis)
//         //         .unwrap();
//         // });

//         // JsFuture::from(promise).await.unwrap();
//     } else {
//         panic!("WASM requires browser environment");
//     }
// }

pub async fn txt_lookup<S: AsRef<str>>(_host: S) -> EyreResult<Vec<String>> {
    bail!("wasm does not support txt lookup")
}

pub async fn ptr_lookup(_ip_addr: IpAddr) -> EyreResult<String> {
    bail!("wasm does not support ptr lookup")
}

pub fn env_variable_is_defined<S: AsRef<str>>(s: S) -> bool {
    false
}

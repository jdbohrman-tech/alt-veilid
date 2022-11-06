use super::utils;
use crate::xx::*;
use crate::*;
use async_executors::{Bindgen, LocalSpawnHandleExt, SpawnHandleExt, Timer};
use futures_util::future::{select, Either};
use js_sys::*;
//use wasm_bindgen_futures::*;
//use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, structural, js_namespace = global, js_name = setTimeout)]
    fn nodejs_global_set_timeout_with_callback_and_timeout_and_arguments_0(
        handler: &::js_sys::Function,
        timeout: u32,
    ) -> Result<JsValue, JsValue>;
}

pub fn get_timestamp() -> u64 {
    if utils::is_browser() {
        return (Date::now() * 1000.0f64) as u64;
    } else {
        panic!("WASM requires browser environment");
    }
}

// pub fn get_timestamp_string() -> String {
//     let date = Date::new_0();
//     let hours = Date::get_utc_hours(&date);
//     let minutes = Date::get_utc_minutes(&date);
//     let seconds = Date::get_utc_seconds(&date);
//     let milliseconds = Date::get_utc_milliseconds(&date);
//     format!(
//         "{:02}:{:02}:{:02}.{}",
//         hours, minutes, seconds, milliseconds
//     )
// }

pub fn random_bytes(dest: &mut [u8]) -> EyreResult<()> {
    let len = dest.len();
    let u32len = len / 4;
    let remlen = len % 4;

    for n in 0..u32len {
        let r = (Math::random() * (u32::max_value() as f64)) as u32;

        dest[n * 4 + 0] = (r & 0xFF) as u8;
        dest[n * 4 + 1] = ((r >> 8) & 0xFF) as u8;
        dest[n * 4 + 2] = ((r >> 16) & 0xFF) as u8;
        dest[n * 4 + 3] = ((r >> 24) & 0xFF) as u8;
    }
    if remlen > 0 {
        let r = (Math::random() * (u32::max_value() as f64)) as u32;
        for n in 0..remlen {
            dest[u32len * 4 + n] = ((r >> (n * 8)) & 0xFF) as u8;
        }
    }

    Ok(())
}

pub fn get_random_u32() -> u32 {
    (Math::random() * (u32::max_value() as f64)) as u32
}

pub fn get_random_u64() -> u64 {
    let v1: u32 = get_random_u32();
    let v2: u32 = get_random_u32();
    ((v1 as u64) << 32) | ((v2 as u32) as u64)
}

pub async fn sleep(millis: u32) {
    Bindgen.sleep(Duration::from_millis(millis.into())).await
}

pub fn system_boxed<'a, Out>(
    future: impl Future<Output = Out> + Send + 'a,
) -> SendPinBoxFutureLifetime<'a, Out> {
    Box::pin(future)
}

pub fn spawn<Out>(future: impl Future<Output = Out> + Send + 'static) -> MustJoinHandle<Out>
where
    Out: Send + 'static,
{
    MustJoinHandle::new(
        Bindgen
            .spawn_handle(future)
            .expect("wasm-bindgen-futures spawn should never error out"),
    )
}

pub fn spawn_local<Out>(future: impl Future<Output = Out> + 'static) -> MustJoinHandle<Out>
where
    Out: 'static,
{
    MustJoinHandle::new(
        Bindgen
            .spawn_handle_local(future)
            .expect("wasm-bindgen-futures spawn_local should never error out"),
    )
}

// pub fn spawn_with_local_set<Out>(
//     future: impl Future<Output = Out> + Send + 'static,
// ) -> MustJoinHandle<Out>
// where
//     Out: Send + 'static,
// {
//     spawn(future)
// }

pub fn spawn_detached<Out>(future: impl Future<Output = Out> + Send + 'static)
where
    Out: Send + 'static,
{
    Bindgen
        .spawn_handle_local(future)
        .expect("wasm-bindgen-futures spawn_local should never error out")
        .detach()
}

pub fn interval<F, FUT>(freq_ms: u32, callback: F) -> SendPinBoxFuture<()>
where
    F: Fn() -> FUT + Send + Sync + 'static,
    FUT: Future<Output = ()> + Send,
{
    let e = Eventual::new();

    let ie = e.clone();
    let jh = spawn(Box::pin(async move {
        while timeout(freq_ms, ie.instance_clone(())).await.is_err() {
            callback().await;
        }
    }));

    Box::pin(async move {
        e.resolve().await;
        jh.await;
    })
}

pub async fn timeout<F, T>(dur_ms: u32, f: F) -> Result<T, TimeoutError>
where
    F: Future<Output = T>,
{
    match select(Box::pin(intf::sleep(dur_ms)), Box::pin(f)).await {
        Either::Left((_x, _b)) => Err(TimeoutError()),
        Either::Right((y, _a)) => Ok(y),
    }
}

// xxx: for now until wasm threads are more stable, and/or we bother with web workers
pub fn get_concurrency() -> u32 {
    1
}

pub async fn get_outbound_relay_peer() -> Option<crate::veilid_api::PeerInfo> {
    // unimplemented!
    None
}

// pub async fn get_pwa_web_server_config() -> {
//     if utils::is_browser() {

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

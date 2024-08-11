#![allow(dead_code)]

use crate::*;

pub async fn get_outbound_relay_peer() -> Option<crate::routing_table::PeerInfo> {
    panic!("Native Veilid should never require an outbound relay");
}

/////////////////////////////////////////////////////////////////////////////////
// Resolver
//
// Uses system resolver on windows and hickory-resolver elsewhere
// hickory-resolver hangs for a long time on Windows building some cache or something
// and we really should be using the built-in system resolver when possible

cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                use async_std_resolver::{config, resolver, AsyncStdResolver as AsyncResolver};
                use hickory_resolver::system_conf::read_system_conf;
            } else if #[cfg(feature="rt-tokio")] {
                use hickory_resolver::{config, TokioAsyncResolver as AsyncResolver, system_conf::read_system_conf};

                async fn resolver(
                    config: config::ResolverConfig,
                    options: config::ResolverOpts,
                ) -> AsyncResolver {
                    AsyncResolver::tokio(config, options)
                }
            } else {
                compile_error!("needs executor implementation")
            }
        }

        struct Resolvers {
            system: Option<Arc<AsyncResolver>>,
            default: Arc<AsyncResolver>,
        }


        lazy_static::lazy_static! {
            static ref RESOLVERS: AsyncMutex<Option<Arc<Resolvers>>> = AsyncMutex::new(None);
        }
    }
}

cfg_if! {
    if #[cfg(not(target_os = "windows"))] {

        async fn with_resolvers<R, F: FnOnce(Arc<Resolvers>) -> SendPinBoxFuture<R>>(closure: F) -> R {
            let mut resolvers_lock = RESOLVERS.lock().await;
            if let Some(r) = &*resolvers_lock {
                return closure(r.clone()).await;
            }

            let (config, mut options) = (config::ResolverConfig::default(), config::ResolverOpts::default());
            options.try_tcp_on_error = true;
            let default = Arc::new(resolver(config, options).await);

            let system = if let Ok((config, options)) = read_system_conf() {
                Some(Arc::new(resolver(config, options).await))
            } else {
                None
            };
            let resolvers = Arc::new(Resolvers {
                system, default
            });
            *resolvers_lock = Some(resolvers.clone());
            closure(resolvers).await
        }

        // async fn reset_resolver(use_default: bool) {
        //     let mut resolver_lock = if use_default {
        //         DEFAULT_RESOLVER.lock().await
        //     } else {
        //         SYSTEM_RESOLVER.lock().await
        //     };
        //     *resolver_lock = None;
        // }

    }
}

pub async fn txt_lookup<S: AsRef<str>>(host: S) -> EyreResult<Vec<String>> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            use core::ffi::c_void;
            use windows::core::{PSTR,PCSTR};
            use std::ffi::{CStr, CString};
            use windows::Win32::NetworkManagement::Dns::{DnsQuery_UTF8, DnsFree, DNS_TYPE_TEXT, DNS_QUERY_STANDARD, DNS_RECORDA, DnsFreeRecordList};

            let mut out = Vec::new();
            unsafe {
                let mut p_query_results: *mut DNS_RECORDA = core::ptr::null_mut();
                let host = CString::new(host.as_ref()).wrap_err("invalid host string")?;
                DnsQuery_UTF8(PCSTR::from_raw(host.as_bytes_with_nul().as_ptr()), DNS_TYPE_TEXT, DNS_QUERY_STANDARD, None, &mut p_query_results as *mut *mut DNS_RECORDA, None).wrap_err("Failed to resolve TXT record")?;

                let mut p_record: *mut DNS_RECORDA = p_query_results;
                while !p_record.is_null() {
                    if (*p_record).wType == DNS_TYPE_TEXT.0 {
                        let count:usize = (*p_record).Data.TXT.dwStringCount.try_into().unwrap();
                        let string_array: *const PSTR = &(*p_record).Data.TXT.pStringArray[0];
                        for n in 0..count {
                            let pstr: PSTR = *(string_array.add(n));
                            let c_str: &CStr = CStr::from_ptr(pstr.0 as *const i8);
                            if let Ok(str_slice) = c_str.to_str() {
                                let str_buf: String = str_slice.to_owned();
                                out.push(str_buf);
                            }
                        }
                    }
                    p_record = (*p_record).pNext;
                }
                DnsFree(Some(p_query_results as *const c_void), DnsFreeRecordList);
            }
            Ok(out)

        } else {
            let host = host.as_ref().to_string();
            let txt_result = with_resolvers(|resolvers| Box::pin(async move {
                // Try the default resolver config
                match resolvers.default
                    .txt_lookup(&host)
                    .await {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            // Try the system resolver config if we have it
                            if let Some(system_resolver) = &resolvers.system {
                                debug!("default resolver txt_lookup error: {}", e);

                                match system_resolver
                                .txt_lookup(&host)
                                .await {
                                    Ok(v) => Ok(v),
                                    Err(e) => {
                                        bail!("system resolver txt_lookup error: {}", e);
                                    }
                                }
                            } else {
                                bail!("default resolver txt_lookup error: {}", e);
                            }
                        }
                    }
                })).await?;
            let mut out = Vec::new();
            for x in txt_result.iter() {
                for s in x.txt_data() {
                    out.push(String::from_utf8(s.to_vec()).wrap_err("utf8 conversion error")?);
                }
            }
            Ok(out)
        }
    }
}

pub async fn ptr_lookup(ip_addr: IpAddr) -> EyreResult<String> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            use core::ffi::c_void;
            use windows::core::{PSTR,PCSTR};
            use std::ffi::{CStr, CString};
            use windows::Win32::NetworkManagement::Dns::{DnsQuery_UTF8, DnsFree, DNS_TYPE_PTR, DNS_QUERY_STANDARD, DNS_RECORDA, DnsFreeRecordList};

            let host = match ip_addr {
                IpAddr::V4(a) => {
                    let oct = a.octets();
                    format!("{}.{}.{}.{}.in-addr.arpa",oct[3],oct[2],oct[1],oct[0])
                }
                IpAddr::V6(a) => {
                    let mut s = String::new();
                    for b in hex::encode(a.octets()).as_bytes().iter().rev() {
                        s.push_str(&format!("{}.",b));
                    }
                    format!("{}ip6.arpa",s)
                }
            };

            unsafe {
                let mut p_query_results: *mut DNS_RECORDA = core::ptr::null_mut();
                let host = CString::new(host).wrap_err("invalid host string")?;
                DnsQuery_UTF8(PCSTR::from_raw(host.as_bytes_with_nul().as_ptr()), DNS_TYPE_PTR, DNS_QUERY_STANDARD, None, &mut p_query_results as *mut *mut DNS_RECORDA, None).wrap_err("Failed to resolve PTR record")?;

                let mut p_record: *mut DNS_RECORDA = p_query_results;
                while !p_record.is_null() {
                    if (*p_record).wType == DNS_TYPE_PTR.0 {
                        let p_name_host: PSTR = (*p_record).Data.PTR.pNameHost;
                        let c_str: &CStr = CStr::from_ptr(p_name_host.0 as *const i8);
                        if let Ok(str_slice) = c_str.to_str() {
                            let str_buf: String = str_slice.to_owned();
                            DnsFree(Some(p_query_results as *const c_void), DnsFreeRecordList);
                            return Ok(str_buf);
                        }
                    }
                    p_record = (*p_record).pNext;
                }
                DnsFree(Some(p_query_results as *const c_void), DnsFreeRecordList);
            }
            bail!("No records returned");
        } else {
            let ptr_result = with_resolvers(|resolvers| Box::pin(async move {
                // Try the default resolver config
                match resolvers.default
                    .reverse_lookup(ip_addr)
                    .await {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            // Try the system resolver config if we have it
                            if let Some(system_resolver) = &resolvers.system {
                                debug!("default resolver ptr_lookup error: {}", e);

                                match system_resolver
                                .reverse_lookup(ip_addr)
                                .await {
                                    Ok(v) => Ok(v),
                                    Err(e) => {
                                        bail!("system resolver ptr_lookup error: {}", e);
                                    }
                                }
                            } else {
                                bail!("default resolver ptr_lookup error: {}", e);
                            }
                        }
                    }
                })).await?;
            if let Some(r) = ptr_result.iter().next() {
                Ok(r.to_string().trim_end_matches('.').to_string())
            } else {
                bail!("PTR lookup returned an empty string");
            }
        }
    }
}

pub fn env_variable_is_defined<S: AsRef<str>>(s: S) -> bool {
    match std::env::var(s.as_ref()) {
        Ok(v) => !v.is_empty(),
        Err(_) => false,
    }
}

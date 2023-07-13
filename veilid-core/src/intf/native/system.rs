#![allow(dead_code)]

use crate::*;

pub async fn get_outbound_relay_peer() -> Option<crate::routing_table::PeerInfo> {
    panic!("Native Veilid should never require an outbound relay");
}

/////////////////////////////////////////////////////////////////////////////////
// Resolver
//
// Uses system resolver on windows and trust-dns-resolver elsewhere
// trust-dns-resolver hangs for a long time on Windows building some cache or something
// and we really should be using the built-in system resolver when possible

cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                use async_std_resolver::{config, resolver, resolver_from_system_conf, AsyncStdResolver as AsyncResolver};
            } else if #[cfg(feature="rt-tokio")] {
                use trust_dns_resolver::{config, TokioAsyncResolver as AsyncResolver, error::ResolveError, error::ResolveErrorKind};

                pub async fn resolver(
                    config: config::ResolverConfig,
                    options: config::ResolverOpts,
                ) -> Result<AsyncResolver, ResolveError> {
                    AsyncResolver::tokio(config, options)
                }

                /// Constructs a new async-std based Resolver with the system configuration.
                ///
                /// This will use `/etc/resolv.conf` on Unix OSes and the registry on Windows.
                #[cfg(any(unix, target_os = "windows"))]
                pub async fn resolver_from_system_conf() -> Result<AsyncResolver, ResolveError> {
                    AsyncResolver::tokio_from_system_conf()
                }
            }
        }


        lazy_static::lazy_static! {
            static ref RESOLVER: Arc<AsyncMutex<Option<AsyncResolver>>> = Arc::new(AsyncMutex::new(None));
        }
    }
}

cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        async fn get_resolver() -> EyreResult<AsyncResolver> {
            let mut resolver_lock = RESOLVER.lock().await;
            if let Some(r) = &*resolver_lock {
                Ok(r.clone())
            } else {
                let resolver = match resolver_from_system_conf().await {
                    Ok(v) => v,
                    Err(_) => resolver(
                        config::ResolverConfig::default(),
                        config::ResolverOpts::default(),
                    )
                    .await
                    .expect("failed to connect resolver"),
                };

                *resolver_lock = Some(resolver.clone());
                Ok(resolver)
            }
        }

        async fn reset_resolver() {
            let mut resolver_lock = RESOLVER.lock().await;
            *resolver_lock = None;
        }

    }
}

pub async fn txt_lookup<S: AsRef<str>>(host: S) -> EyreResult<Vec<String>> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            use core::ffi::c_void;
            use windows::core::PSTR;
            use std::ffi::CStr;
            use windows::Win32::NetworkManagement::Dns::{DnsQuery_UTF8, DnsFree, DNS_TYPE_TEXT, DNS_QUERY_STANDARD, DNS_RECORDA, DnsFreeRecordList};

            let mut out = Vec::new();
            unsafe {
                let mut p_query_results: *mut DNS_RECORDA = core::ptr::null_mut();
                let status = DnsQuery_UTF8(host.as_ref(), DNS_TYPE_TEXT as u16, DNS_QUERY_STANDARD, core::ptr::null_mut(), &mut p_query_results as *mut *mut DNS_RECORDA, core::ptr::null_mut());
                if status != 0 {
                    bail!("Failed to resolve TXT record");
                }

                let mut p_record: *mut DNS_RECORDA = p_query_results;
                while !p_record.is_null() {
                    if (*p_record).wType == DNS_TYPE_TEXT as u16 {
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
                DnsFree(p_query_results as *const c_void, DnsFreeRecordList);
            }
            Ok(out)

        } else {
            let resolver = get_resolver().await?;
            let txt_result = match resolver
                .txt_lookup(host.as_ref())
                .await {
                    Ok(v) => v,
                    Err(e) => {
                        if matches!(e.kind(), ResolveErrorKind::NoConnections) {
                            reset_resolver().await;
                        }
                        bail!("txt_lookup error: {}", e);
                    }
                };
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
            use windows::core::PSTR;
            use std::ffi::CStr;
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
                let status = DnsQuery_UTF8(host, DNS_TYPE_PTR as u16, DNS_QUERY_STANDARD, core::ptr::null_mut(), &mut p_query_results as *mut *mut DNS_RECORDA, core::ptr::null_mut());
                if status != 0 {
                    bail!("Failed to resolve PTR record");
                }

                let mut p_record: *mut DNS_RECORDA = p_query_results;
                while !p_record.is_null() {
                    if (*p_record).wType == DNS_TYPE_PTR as u16 {
                        let p_name_host: PSTR = (*p_record).Data.PTR.pNameHost;
                        let c_str: &CStr = CStr::from_ptr(p_name_host.0 as *const i8);
                        if let Ok(str_slice) = c_str.to_str() {
                            let str_buf: String = str_slice.to_owned();
                            return Ok(str_buf);
                        }
                    }
                    p_record = (*p_record).pNext;
                }
                DnsFree(p_query_results as *const c_void, DnsFreeRecordList);
            }
            bail!("No records returned");
        } else {
            let resolver = get_resolver().await?;
            let ptr_result = resolver
                .reverse_lookup(ip_addr)
                .await
                .wrap_err("resolver error")?;
            if let Some(r) = ptr_result.iter().next() {
                Ok(r.to_string().trim_end_matches('.').to_string())
            } else {
                bail!("PTR lookup returned an empty string");
            }
        }
    }
}

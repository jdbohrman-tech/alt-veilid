use super::*;

mod permutation;
mod remote_private_route_info;
mod route_set_spec_detail;
mod route_spec_store;
mod route_spec_store_cache;
mod route_spec_store_content;
mod route_stats;

pub use permutation::*;
pub use remote_private_route_info::*;
pub use route_set_spec_detail::*;
pub use route_spec_store::*;
pub use route_spec_store_cache::*;
pub use route_spec_store_content::*;
pub use route_stats::*;

use crate::veilid_api::*;
use rkyv::{
    with::Skip, Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};

/// The size of the remote private route cache
const REMOTE_PRIVATE_ROUTE_CACHE_SIZE: usize = 1024;
/// Remote private route cache entries expire in 5 minutes if they haven't been used
const REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY: TimestampDuration = TimestampDuration::new(300_000_000u64);
/// Amount of time a route can remain idle before it gets tested
const ROUTE_MIN_IDLE_TIME_MS: u32 = 30_000;
/// The size of the compiled route cache
const COMPILED_ROUTE_CACHE_SIZE: usize = 256;

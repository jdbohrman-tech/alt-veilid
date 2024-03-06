use super::*;

/// Watch parameters used to configure a watch
#[derive(Debug, Clone)]
pub struct WatchParameters {
    /// The range of subkeys being watched, empty meaning full
    pub subkeys: ValueSubkeyRangeSet,
    /// When this watch will expire
    pub expiration: Timestamp,
    /// How many updates are left before forced expiration
    pub count: u32,
    /// The watching schema member key, or an anonymous key
    pub watcher: PublicKey,
    /// The place where updates are sent
    pub target: Target,
}

/// Watch result to return with answer
/// Default result is cancelled/expired/inactive/rejected
#[derive(Debug, Clone)]
pub enum WatchResult {
    /// A new watch was created
    Created {
        /// The new id of the watch
        id: u64,
        /// The expiration timestamp of the watch. This should never be zero.
        expiration: Timestamp,
    },
    /// An existing watch was modified
    Changed {
        /// The new expiration timestamp of the modified watch. This should never be zero.
        expiration: Timestamp,
    },
    /// An existing watch was cancelled
    Cancelled,
    /// The request was rejected due to invalid parameters or a missing watch
    Rejected,
}

/// An individual watch
#[derive(Debug, Clone)]
pub struct Watch {
    /// The configuration of the watch
    pub params: WatchParameters,
    /// A unique id per record assigned at watch creation time. Used to disambiguate a client's version of a watch
    pub id: u64,
    /// What has changed since the last update
    pub changed: ValueSubkeyRangeSet,
}

#[derive(Debug, Default, Clone)]
/// A record being watched for changes
pub struct WatchList {
    /// The list of active watches
    pub watches: Vec<Watch>,
}

/// How a watch gets updated when a value changes
pub enum WatchUpdateMode {
    /// Update no watchers
    NoUpdate,
    /// Update all watchers
    UpdateAll,
    /// Update all watchers except ones that come from a specific target
    ExcludeTarget(Target),
}

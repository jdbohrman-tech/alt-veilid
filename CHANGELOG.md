**UNRELEASED**

- Update capnproto version to 1.1.0
- *BREAKING API CHANGE*:
  - watch_dht_values() now returns a bool rather than an expiration timestamp. Expiration renewal is now managed by veilid-core internally. Apps no longer need to renew watches!
  - inspect_dht_record() and cancel_dht_watch() now take an Option<ValueSubkeyRangeSet> instead of just a ValueSubkeyRangeSet, to make things easier for automatic binding generation someday and to remove ambiguities about the semantics of the default empty set.
  - DHTRecordReport now uses a Vec<Option<ValueSubkey>> for seq lists, rather than using the 'ValueSubkey::MAX' sentinel value (0xFFFFFFFF) to represent a missing subkey
  - Renamed config structs to better describe their purpose, and remove "Inner" from a struct that's being exposed via the API. ([!402](https://gitlab.com/veilid/veilid/-/merge_requests/402))
    - `VeilidConfig` -> `VeilidStartupOptions`
    - `VeilidConfigInner` -> `VeilidConfig`
  - Gate insecure capabilities behind the new `footgun` feature flag ([#394](https://gitlab.com/veilid/veilid/-/issues/394)), which is disabled by default. ([!400](https://gitlab.com/veilid/veilid/-/merge_requests/400))
    - Calling `app_call` or `app_message` with a `NodeId` target will throw an error. Use a `PrivateRoute` target instead.
    - Creating an `Unsafe` routing context will throw and error. Use `Safe` routing context instead.
    - Any AppCall or AppMessage sent from a direct NodeId will not be received.

- veilid-core:
  - **Security** Signed bootstrap v1 added which closes #293: https://gitlab.com/veilid/veilid/-/issues/293
  - Allow shutdown even if tables are closed
  - New, more robust, watchvalue implementation
  - Consensus is now counted from the nodes closest to the key, excluding attempts that have failed, but including new nodes that show up, requiring N out of the M closest nodes to have      succeeded and all have been attempted.
  - Watching a node now also triggers an background inspection+valueget to detect if values have changed online
  - Fanout queue disqualifaction for distance-based rejections reimplemented
  - Local rehydration implemented. DHT record subkey data that does not have sufficient consensus online is re-pushed to keep it alive when records are opened.
  - Direct bootstrap v0 now filters out Relayed nodes correctly
  - Closed issue #400: https://gitlab.com/veilid/veilid/-/issues/400
  - Closed issue #377: https://gitlab.com/veilid/veilid/-/issues/377
  - Add the `veilid_features()` API, which lists the compile-time features that were enabled when `veilid-core` was built (available in language bindings as well). ([!401](https://gitlab.com/veilid/veilid/-/issues/400))
  - When `veilid-core` starts up, log the version number, and the compile-time features that were enabled when it was built. ([!401](https://gitlab.com/veilid/veilid/-/issues/400))
  - Closed issue #448: https://gitlab.com/veilid/veilid/-/issues/448
  - Add background flush for routing table and route spec store, to address issue #449: https://gitlab.com/veilid/veilid/-/issues/449

- veilid-flutter:
  - Bindings updated for API changes
  - Corrosion version in cmake build for linux and windows updated to 0.5.1: https://gitlab.com/veilid/veilid/-/issues/447
  - Expose the isShutdown API: https://gitlab.com/veilid/veilid/-/merge_requests/392

- veilid-python:
  - Fix type assertion bug in watch_dht_values
  - Update watchvalue integration tests
  - Expose the is_shutdown API: https://gitlab.com/veilid/veilid/-/merge_requests/392

- veilid-wasm:
  - **Breaking** Properly generate TypeScript types for `ValueSubkeyRangeSet`, which would previously resolve to `any`. This is breaking since it can cause type errors to correctly surface in existing applications. ([!397](https://gitlab.com/veilid/veilid/-/merge_requests/397))
  - **Breaking** `startupCore()` and `defaultConfig()` now use config objects instead of stringified JSON.
    - `veilidClient.startupCore(callback, JSON.stringify(config))` now becomes `veilidClient.startupCore(callback, config)`. ([!402](https://gitlab.com/veilid/veilid/-/merge_requests/402))
    - `JSON.parse(veilidClient.defaultConfig())` is now `veilidClient.defaultConfig()`
    - The `VeilidConfigInner` type is now `VeilidConfig`.
  - Expose the isShutdown API: https://gitlab.com/veilid/veilid/-/merge_requests/392

- CI:
  - Ensure Cargo.lock is up-to-date during CI pipelines

**Changed in Veilid 0.4.4**

- veilid-core:
  - Improved termination conditions for DiscoveryContext, faster shutdown acknowledgment
  - Network class discovery from 25s to 5s in worst case
  - Moved some futures to the heap with Box::pin to reduce risk of stack overflows
  - Restore punishment for FailedToVerifySenderPeerInfo
  - Convert inbound relay loop notification from an error to a NetworkResult
  - Relays were being picked before we had good stats on their performance. Relays are now optimized to be within the top 25% of p90 latencies
  - LatencyStats now have median 80% average, p90, p75, and p50 latency numbers
  - RPC Answer stats now are split by 'ordered' and 'unordered' protocols
  - Unordered protocols are given a higher tolerance for lost answers before being marked 'unreliable'
  - Fix race condition in TTL code for udp hole punching
  - Correct use of IPV6_UNICAST_HOPS socket option instead of IP_TTL for ipv6 sockets
  - Move relaying to a relay worker pool
  - Connection table and RPC locking improvements
  - Remove TCP connection retries
  - Deadlock bugfix in relay code
  - 'Finding more nodes' tasks now use PreferOrdered sequencing
  - Fix deadlock in veilid_api duration testing
  - Correct offline_subkey_writes inflight reporting
  - Simplify upnp code, eliminate useless high-latency retry logic, and ensure it doesn't conflict with NAT detection

- veilid-python:
  - Fix async context implementation so it works correctly when re-entrant (nested `async with` blocks)
  - Validate types for API calls, will raise an assertion error if type is unexpected
  - Add more stress tests

- veilid-wasm:
  - fork tracing-wasm and add output filter for veilid log key

- general:
  - Added some pedantic clippy lints, moved lint config into workspace Cargo.toml
  - Fixed async-std support
  - Added windows and macos clippy lints to CI via earthfile
  - Upgrade all pinned crate dependencies

- CICD
  - Began migrating build/package/deploy scripts from shell to Python
  - Switched from using multiple build machines (one per arch/package) to a single machine running the +package-linux Earthly target

- _community contributions_
  - (veilid-wasm) Remove wee_alloc due to memory leak issues -- @bmv437
  - (veilid-core) Fix geolocation feature after recent refactor -- @neequ57
  - (documentation) Update README code snippet to work with current version -- @carvilsi


**Changed in Veilid 0.4.3**

This release exists without changes to the Veilid codebase in order to test fixes to the CICD release pipeline.

- CICD
  - Fixed broken RPM packaging

**Changed in Veilid 0.4.2**

veilid-core:
- (neequ57) Merged !330 - geolocation feature (off by default) to allow excluding/denylisting route nodes based on geography
- (evelyn) Merged !267 - adding the ability to create dht records with a specified owner key (rust only currently)
- (rivka segan) Merged !335 - fix logic error that used wss when not tls
- New startup/shutdown initialization and component system
- Logs are tagged with the program_name+namespace they are collected in
  - No more per-facility log macros, one unified `veilid_log!()` macro for all events
  - Switch between subnodes in veilid-cli switches which logs you're viewing
  - All global logs and subnode 0 go to console, all other subnodes are accessible via veilid-cli
- Major refactor to add VeilidComponentRegistry as the 'owner' of all components
  - Access to components now uses scoped guards for lifetime management rather than loose Arc clones
  - VeilidComponent trait makes adding common per-component functions easier
  - Unified initialize, post-initialize, pre-terminate, and terminate phase harness
- AsyncCryptoSystemGuard added to make heavy operations happier in async environments
- UDP hole punch needed TTL setting to keep routers from incorrectly making conntracks
- Public address detection was getting stuck in a lock contention, regression from refactor
- PeerInfo caching to eliminate some repeated cloning
- NodeContactMethod cache improvements
- Symmetric NAT and NetworkClass::OutboundOnly were broken. When routing domain address types are known, but there is no dialinfo, that should be OutboundOnly and not Invalid. It's valid to have no dialinfo. Added network class 'confirmation'.

veilid-tools:
- replaced deprecated serde_yaml crate with maintained serde_yaml_ng crate
- Start of VirtualRouter network virtualization
  - standalone virtual router binary in veilid-tools (`cargo run --bin virtual_router`)
  - IAC-style configuration system for repeatable virtualized network generation

veilid-flutter:
- (kimmy.zip) Merged !343 - Fixes for Windows Flutter build
- Android NDK version requirement is now 27.0.12077973
- Android Gradle version is now 8.10.2, with a minimum of 8.8.0
- Android Java version is now 17
- rust-android-gradle upgraded to 0.9.6
- Kotlin version is now 1.9.25
- API added for create_dht_record with 'owner'
  - Breaking change: https://gitlab.com/veilid/veilid/-/merge_requests/353

veilid-cli:
- You can now switch between subnodes easily with the 'connect <N>' command where N is the subnode id

veilid-server:
- You can now run multiple subnodes concurrently in the same process with `--subnode_count=N`
- Up to 256 concurrent each of TCP and WebSocket connections now, up from 32
- Turn off detect_address_changes and upnp by default

veilid-wasm:
- (bgrift) Merged !352 - WASM supports owner on createDhtRecord, also added the getDhtRecordKey function
  - Breaking change: https://gitlab.com/veilid/veilid/-/merge_requests/352
- Fixes for heavy sync crypto code, optimizations in debug mode, wasm tests went from 731 seconds to 112 seconds

veilid-python:
- API added for create_dht_record with 'owner'
  - Breaking change: https://gitlab.com/veilid/veilid/-/merge_requests/353
- api_connector() now attempts IPC connection to veilid-server before trying port 5959 tcp
- dependencies corrected for pypi package

CICD:
- Updated build machines
  - OS Updates
  - Rust to 1.81
  - Python to 3.12
  - Earthly to 0.8.15
 
general:
- Fix rust-version into workspace cargo.toml
- Earthfile update to 0.8
- Earthfile cache efficiency fixes

**Changed in Veilid 0.4.1**

- Implement top level event bus to do asynchronous lock-free communication between subsystems
- Fix deadlock in socket address change event
- Fix deadlock in peer info change event
- Fix incorrect node info equivalence check
- Ping relays every second instead of every 10 seconds
- MR !328 'tiny improvements'

**Changed in Veilid 0.4.0**

- RFC-0001: Constrain DHT Subkey Size, issue #406

- Dialinfo detection issues:
  - Add a publish() as well as a commit() for routing domain editor
  - Should only publish our peer info after we're sure we done editing it (end of public address detection task)
  - Publish should happen after relay selection as well
  - Publish should happen if the relay's peerinfo has changed
  - Publish should not do anything if the peerinfo hasn't changed
  - PeerInfo -> Arc<PeerInfo> everywhere to minimize deep clones and ensure read-only PeerInfo
  - Routing domain editing is now more atomic
  - When a node selects a relay it now immediately protects its connections.
  - Made dial info port (for port restricted nat) more resilient to changes, in the case there are multiple mappings
  - Relays that drop protected connections should be deprioritized for relay selection (table saturation detection)
  - clear_network_callback in do_public_dial_info_check is a kludge, removed
  - Raised the bar for dialinfo changes when its just the port
  - Pinging node on the same network works again
  - resolve_node() never returns a dead node even when we want to try to communicate with it again
  - Removed 'bad public address' detection as it wasn't working anyway
  - Added separate parallelism lanes for relay keepalive pings from peer liveness check pings, as they are higher priority
  - Change send_data to always check cache for contact method first instead of going with filtered active flows first, avoids choosing UDP when a preferable TCP connection could be made
  - Nodes that are not relay capable should drop relayed packets

- DHT issues:
  - Make setvalue more likely to succeed by accepting a getvalue consensus if a full setvalue consensus is not reached.
  - Offline subkey writes are cleared too fast and should be thought as 'subkeys not yet synchronized'
  - If set_value is partial / in-flight, it should still be in offline_subkey_writes
  - Make inflight_subkey_writes list and probably some bit for 'written_while_inflight' so we dont clear the offline_subkey_writes until they're really written

- Networking:
  - Fix TIME_WAIT states on Windows
  - Attempt to give priority to relaying flows

- UI:
  - Make veilid-cli display the connection state when reconnecting, and exit more cleanly on ctrl-c
  - Added 'uptime' veilid-cli debug command

- Misc:
  - Fixes for python DHT test

- API Additions:
  - VeilidConfigInner::new parameteriztion for easier config from rust apps
  - Remove veilid-server specific paths from veilid-core defaults
  - Lots more stats about node performance in PeerStats
  - Uptime stats in VeilidStateAttachment/VeilidUpdateAttachment, issue #317

**Changed in Veilid 0.3.4**
- Crates updates
  - Update crates to newer versions
  - Remove veilid-async-tungstenite and veilid-async-tls crates as they are no longer needed
- Fix startup/shutdown/attach/detach
  - Improved resource accounting
  - Locked startup/shutdown mechanism
  - Perfetto profiler output for ui.perfetto.dev
  - SO_LINGER(0) re-enabled to eliminate TIME_WAIT on restart/detach/attach
  (this may cause noise for WASM in browsers when websockets are RST dropped rather than handshake closed, we will deal with this later)
- _Community Contributions_
  - prototype script to install / run a veilid-server node within a unifyos device (tested on a unify dream machine pro SE) @Vyrus-001

**Changed in Veilid 0.3.3**
- Fix set_dht_value and watch_value
  - Watching values incorrectly categorized 'owner' keys as anonymous watchers
  - Setting a dht value with the same sequence number as what is on the network, but with a conflicting value, did not result in the current value being returned to the api caller as it should have been
- DHT cleanup
  - Proper application of DHT capabilities
  - Fanout debugging log target
  - Performance measurement / timing of veilid_api log target
- Fix DHT Rust integration test
- ValueChanged Optional
  - Allow value changed data to be optional in rpc schema
  - Make valuechanged update no longer happen when value hasn't changed or is older
- Implement closest peers refresh
  - Implement closest peers refresh. Closes issue #372.
  - Find_self/find_target can use capability check
  - Fix offline subkey write reporting to eliminate spurious notifications
  - Add more detail to public address check
- Improved punishment and state
  - Create 'reasons' for dead and unreliable states
  - Make 'punished' its own state
  - Closes issue #281
  - Fixes an issue with reliable nodes being marked as 'dead' unjustly
- _Community Contributions_
  - Fixed memory leak in Windows DNS resolver @kyanha

**Changed in Veilid 0.3.2**
- DHT optimization and bugfixes
  - Support for offline write status in DHTRecordReport
  - Fix deprecated functions
  - Improve fanout seeding to ensure records are reached as quickly as possible
- Native IPV4-IPV6 bridging support
  - fix bug where messages sent to a private route without a safety route would not receive replies
  - fix verbose-tracing feature flag
  - improve route allocation to avoid co-located nodes
  - fix contact method for nodes on the same ipblock
  - add support for maintaining AddressType-translation relays
- Removed NDK related hotfix, as this has been integrated into cargo-ndk already
- Open sourced the CICD build scripts
- Fixes for watchvalue
- Refactor low level network

**Changed in Veilid 0.3.1**
- DHT cleanup
  - Proper application of DHT capabilities
  - Fanout debugging log target
  - Performance measurement / timing of veilid_api log target
- ValueChanged Optional
  - Allow value changed data to be optional in rpc schema
  - Make valuechanged update no longer happen when value hasn't changed or is older
- Clippy fixes and cleanup
- _Community Contributions_
  - Changed VeilidAPI::parse_as_target to a sync function -- @sashanoraa
  - fix dht rust integration test -- @ssurovsev

**Changed in Veilid 0.3.0**
- API BREAKING CHANGES: 
  - WatchValue RPC support
  - InspectRecord RPC support
  - RoutingContext now defaults to Reliable and EnsureOrdered modes
  - generate_shared_secret added that abstracts DH and ensures domain separation
- Closed #357 - AppCall and AppMessage now have private route information
- Logging: Log facilities now can be enabled and disabled at runtime
- Logging: Log facility added for DHT, network results, and API calls
- CLI: Closed #358 - veilid-cli now has 'interactive' (-i), 'log viewer' (-l) and 'execute command' (-e) command line options
- Testing: veilid-flutter now has integration tests of its own that work like the veilid-python unit tests
- Network: Failures to hole-punch UDP or reverse-connect TCP or UDP now falls back to inbound relaying
- Bugfix: Signal handling for unix-like platforms was not handling SIGTERM correctly
- Bugfix: Restarting veilid-server quickly might result in failures to bind()
- Bugfix: Closed #359 - Block node identity from DHT record schema owner/writer
- Bugfix: Closed #355 - Fixed memory error reading macos/ios interfaces list
- _Community Contributions_
  - Made private route allocation bidirectional by default @kyanha
  - Use $CI_REGISTRY_IMAGE for the registry path @SalvatoreT
  - Add VeilidConfigInner-based VeilidAPI startup @SalvatoreT
  - rebrand trust-dns-resolver to hickory-resolver @kyanha

**Changed in Veilid 0.2.5**
- API BREAKING CHANGES: 
  - on `RoutingContext`: `with_privacy()` renamed to `with_default_safety()`
  - on `RoutingContext`: `with_custom_privacy()` renamed to `with_safety()`
  - on `RoutingContext`: `safety()` method added that returns the current `SafetySelection`
  - Routing contexts are now safety-route-enabled by default. To disable, use `with_safety()` with `SafetySelection::Unsafe`.
- WASM now works better with updated connection manager code
- Async-std flavor of veilid-core now builds correctly again
- Safety route allocation is bidirectional
- Connection table LRU cache now has protection for relays and in-use RPC question/answers
- Dead route notifications are now sent only for manually allocated routes
- Allocated routes that fail tests now have their nodes marked as 'failure to send' so they go 'unreliable' and get re-tested. Also the same route will not immediately be reallocated as a result.
- DHT tests ported from Python to Rust
- Rustls updated to latest release
- Protected connections (such as relays) that drop result in marking the node as 'failure to send' so a different relay gets chosen

**Changed in Veilid 0.2.4**
- Fixed issue with client API failing when ipv6 was disabled
- Android fixed so it can move out of invalid network state
- Numerous WASM binding fixes
- IGD/UPNP fixes for Windows
- Reduce network downtime when local ip addresses change (ipv6 temporary addresses)
- Fix support for Android emulator
- Bootstrap is more robust in environments where some dialinfo won't work, like inbound UDP being firewalled off
- CLI has timestamps in the log output
- Base64 fixes for encoding
- IPv6 capability detection for native platforms

**Changed in Veilid 0.2.3**
- Security fix for WS denial of service
- Support for latest Rust 1.72

**Changed in Veilid 0.2.2**
- Capnproto 1.0.1 + Protobuf 24.3
- DHT set/get correctness fixes
- Connection table fixes
- Node resolution fixes
- More debugging commands (appmessage, appcall, resolve, better nodeinfo, etc)
- Reverse connect for WASM nodes
- Better Typescript types for WASM
- Various script and environment cleanups
- Earthly build for aarch64 RPM
- Much improved and faster public address detection

**Changes in Veilid 0.2.1**
- Crates are separated and publishable
- First publication of veilid-core with docs to crates.io and docs.rs
- Avoid large logs of 127.0.0.1:5959 attack payloads
- Use getrandom in WASM for RNG
- Increase privacy for WASM builds by rewriting internal paths
- Translations
- Fix python update schema script
- Earthfile cleanup

**Changes in Veilid 0.2.0**
- Rustdoc builds now
- API visibility changes
- Android JNI update
- Fix DHT record data housekeeping
- Public address detection improvement
- Manual port forwarding detection 
- lock_api dependency fix
- DialInfo failover when some dial info does not work

Note: Windows builds may be broken in this release. Please test and let us know by opening an issue.

**Changes in Veilid 0.1.10**
- BREAKING CHANGE: ALL MUST UPDATE
  * VLD0 now adds a BLAKE3 hash round on the DH output to further separate it from the raw key exchange
  * Bootstraps are fixed now due to DH issue
- Windows crate update caused build and nul termination issues for DNS resolver
- Fix for network key on the veilid-server command line
- Strict verification for Ed25519 enabled
- Domain separation for VLD0 signing and crypt
  
**Changes in Veilid 0.1.9**
- SECURITY FIX
  * DESCRIPTION: Decompression was occurring in an unbounded way upon envelope receipt.
  * IMPACT: Node crashes resulting in downtime. There was no risk of RCE or compromise due to Rust's memory protections and no use of unsafe code near the site of the error.
  * INDICATIONS: This resulted in an out-of-memory abort on nodes. Issue first identified on the bootstrap servers. 
  * REMEDIATION: Length check added to decompression on envelopes.
- Earthfile support for generating a debug executable

**Changes in Veilid 0.1.8**
- Fix Python Install Instructions
- Fix to get server version from crate
- Move dev setup into its own folder
- Setup support for Fedora
- Make submodule paths absolute
- veilid-flutter improvements for crypto and timestamp, and endianness bugfix
- Offline subkey writes for DHT
- Fix WASM compilation
- Improve server port allocation
- Add more punishments
- Clap derive refactor for command line args
- gitignore emacs backup files
- Various typos
- Fanout debugging for DHT

**Changes in Veilid 0.1.7**

- Fix for connection table crash
- Fix for incorrect set_dht_value return value
- Python test updates
- Various VeilidChat-prompted veilid-flutter updates

**Changes in Veilid 0.1.6**

- Fix for 'find_node' too many nodes returned issue

**Changes in Veilid 0.1.5**

- Added Changelog 
- Fix detachment issue with suspending network interfaces during operation
- Fix incorrect punishment on relayed undecryptable messages
- Minor API feature adds
- Relay bugfixes

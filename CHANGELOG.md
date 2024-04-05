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

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

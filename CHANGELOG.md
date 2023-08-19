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

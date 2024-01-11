# Welcome to Veilid

- [From Orbit](#from-orbit)
- [Run a Node](#run-a-node)
- [Development](#development)

## From Orbit

The first matter to address is the question "What is Veilid?" The highest-level description is that Veilid is a peer-to-peer network for easily sharing various kinds of data.

Veilid is designed with a social dimension in mind, so that each user can have their personal content stored on the network, but also can share that content with other people of their choosing, or with the entire world if they want.

The primary purpose of the Veilid network is to provide the infrastructure for a specific kind of shared data: social media in various forms. That includes light-weight content such as Twitter's tweets or Mastodon's toots, medium-weight content like images and songs, and heavy-weight content like videos. Meta-content such as personal feeds, replies, private messages, and so forth are also intended to run atop Veilid.

## Run a Node

The easiest way to help grow the Veilid network is to run your own node. Every user of Veilid is a node, but some nodes help the network more than others. These network support nodes are heavier than the node a user would establish on their phone in the form of a chat or social media application. A cloud based virtual private server (VPS), such as Digital Ocean Droplets or AWS EC2, with high bandwidth, processing resources, and up time availability is crucial for building the fast, secure, and private routing that Veilid is built to provide.

To run such a node, establish a Debian or Fedora based VPS and install the veilid-server service. To make this process simple we are hosting package manager repositories for .deb and .rpm packages. See the [installing](./INSTALL.md) guide for more information.

## Building on Veilid

If you want to start using Veilid for your own app, take a look at the [Developer Book](https://veilid.gitlab.io/developer-book/).

A basic example using `veilid-core` and `tokio` might look like this.

```rust
use std::sync::Arc;
use veilid_core::VeilidUpdate::{AppMessage, Network};
use veilid_core::{VeilidConfigBlockStore, VeilidConfigInner, VeilidConfigProtectedStore, VeilidConfigTableStore, VeilidUpdate};

#[tokio::main]
async fn main() {
    let update_callback = Arc::new(move |update: VeilidUpdate| {
        match update {
            AppMessage(msg) => {
                println!("Message: {}", String::from_utf8_lossy(msg.message().into()));
            }
            Network(msg) => {
                println!("Network: Peers {:}, bytes/sec [{} up] [{} down]", msg.peers.iter().count(), msg.bps_up, msg.bps_down)
            }
            _ => {
                println!("{:?}", update)
            }
        };
    });

    let config = VeilidConfigInner {
        program_name: "Example Veilid".into(),
        namespace: "veilid-example".into(),
        protected_store: VeilidConfigProtectedStore {
            // avoid prompting for password, don't do this in production
            always_use_insecure_storage: true,
            directory: "./.veilid/block_store".into(),
            ..Default::default()
        },
        block_store: VeilidConfigBlockStore {
            directory: "./.veilid/block_store".into(),
            ..Default::default()
        },
        table_store: VeilidConfigTableStore {
            directory: "./.veilid/table_store".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let veilid = veilid_core::api_startup_config(update_callback, config).await.unwrap();
    println!("Node ID: {}", veilid.config().unwrap().get_veilid_state().config.network.routing_table.node_id);
    veilid.attach().await.unwrap();
    // Until CTRL+C is pressed, keep running
    tokio::signal::ctrl_c().await.unwrap();
    veilid.shutdown().await;
}
```

## Development

If you're inclined to get involved in code and non-code development, please check out the [contributing](./CONTRIBUTING.md) guide. We're striving for this project to be developed in the open and by people for people. Specific areas in which we are looking for help include:

- Rust
- Flutter/Dart
- Python
- Gitlab DevOps and CI/CD
- Documentation
- Security reviews
- Linux packaging

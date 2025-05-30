use std::sync::Arc;
use veilid_core::VeilidUpdate::{AppMessage, Network};
use veilid_core::{VeilidConfig, VeilidConfigProtectedStore, VeilidConfigTableStore, VeilidUpdate};

#[tokio::main]
async fn main() {
    let update_callback = Arc::new(move |update: VeilidUpdate| {
        match update {
            AppMessage(msg) => {
                println!("Message: {}", String::from_utf8_lossy(msg.message()));
            }
            Network(msg) => {
                println!(
                    "Network: Peers {:}, bytes/sec [{} up] [{} down]",
                    msg.peers.len(),
                    msg.bps_up,
                    msg.bps_down
                )
            }
            _ => {
                println!("{:#?}", update)
            }
        };
    });

    let config = VeilidConfig {
        program_name: "Example Veilid".into(),
        namespace: "veilid-example".into(),

        protected_store: VeilidConfigProtectedStore {
            // IMPORTANT: don't do this in production
            // This avoids prompting for a password and is insecure
            always_use_insecure_storage: true,
            directory: "./.veilid/protected_store".into(),
            ..Default::default()
        },
        table_store: VeilidConfigTableStore {
            directory: "./.veilid/table_store".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let veilid = veilid_core::api_startup_config(update_callback, config)
        .await
        .unwrap();
    println!(
        "Node ID: {}",
        veilid.config().unwrap().get().network.routing_table.node_id
    );
    veilid.attach().await.unwrap();
    // Until CTRL+C is pressed, keep running
    tokio::signal::ctrl_c().await.unwrap();
    veilid.shutdown().await;
}

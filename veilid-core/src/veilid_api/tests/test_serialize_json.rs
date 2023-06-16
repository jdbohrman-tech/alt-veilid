use super::test_types::*;
use super::test_types_dht::*;
use super::test_types_dht_schema::*;

pub async fn test_all() {
    // test_types
    test_alignedu64().await;
    test_veilidappmessage().await;
    test_veilidappcall().await;
    test_fourcc().await;
    test_sequencing().await;
    test_stability().await;
    test_safetyselection().await;
    test_safetyspec().await;
    test_latencystats().await;
    test_transferstats().await;
    test_transferstatsdownup().await;
    test_rpcstats().await;
    test_peerstats().await;
    #[cfg(feature = "unstable-tunnels")]
    test_tunnelmode().await;
    #[cfg(feature = "unstable-tunnels")]
    test_tunnelerror().await;
    #[cfg(feature = "unstable-tunnels")]
    test_tunnelendpoint().await;
    #[cfg(feature = "unstable-tunnels")]
    test_fulltunnel().await;
    #[cfg(feature = "unstable-tunnels")]
    test_partialtunnel().await;
    test_veilidloglevel().await;
    test_veilidlog().await;
    test_attachmentstate().await;
    test_veilidstateattachment().await;
    test_peertabledata().await;
    test_veilidstatenetwork().await;
    test_veilidroutechange().await;
    test_veilidstateconfig().await;
    test_veilidvaluechange().await;
    test_veilidupdate().await;
    test_veilidstate().await;
    // test_types_dht
    test_dhtrecorddescriptor().await;
    test_valuedata().await;
    test_valuesubkeyrangeset().await;
    // test_types_dht_schema
    test_dhtschemadflt().await;
    test_dhtschema().await;
    test_dhtschemasmplmember().await;
    test_dhtschemasmpl().await;
}

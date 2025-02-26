use super::test_types::*;
use super::test_types_dht::*;
use super::test_types_dht_schema::*;

#[expect(clippy::unused_async)]
pub async fn test_all() {
    // test_types
    test_alignedu64();
    test_veilidappmessage();
    test_veilidappcall();
    test_fourcc();
    test_sequencing();
    test_stability();
    test_safetyselection();
    test_safetyspec();
    test_latencystats();
    test_transferstats();
    test_transferstatsdownup();
    test_rpcstats();
    test_peerstats();
    #[cfg(feature = "unstable-tunnels")]
    test_tunnelmode();
    #[cfg(feature = "unstable-tunnels")]
    test_tunnelerror();
    #[cfg(feature = "unstable-tunnels")]
    test_tunnelendpoint();
    #[cfg(feature = "unstable-tunnels")]
    test_fulltunnel();
    #[cfg(feature = "unstable-tunnels")]
    test_partialtunnel();
    test_veilidloglevel();
    test_veilidlog();
    test_attachmentstate();
    test_veilidstateattachment();
    test_peertabledata();
    test_veilidstatenetwork();
    test_veilidroutechange();
    test_veilidstateconfig();
    test_veilidvaluechange();
    test_veilidupdate();
    test_veilidstate();
    // test_types_dht
    test_dhtrecorddescriptor();
    test_valuedata();
    test_valuesubkeyrangeset();
    // test_types_dht_schema
    test_dhtschemadflt();
    test_dhtschema();
    test_dhtschemasmplmember();
    test_dhtschemasmpl();
}

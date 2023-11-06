use super::test_veilid_config::*;
use crate::*;

use lazy_static::*;

lazy_static! {
    static ref BOGUS_KEY: TypedKey = TypedKey::from(CryptoTyped::new(
        CRYPTO_KIND_VLD0,
        CryptoKey::new([0u8; 32])
    ));
}

pub async fn test_get_dht_value_unopened(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let result = rc.get_dht_value(*BOGUS_KEY, 0, false).await;
    assert_err!(result);
}

pub async fn test_open_dht_record_nonexistent_no_writer(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let result = rc.get_dht_value(*BOGUS_KEY, 0, false).await;
    assert_err!(result);
}

pub async fn test_close_dht_record_nonexistent(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let result = rc.close_dht_record(*BOGUS_KEY).await;
    assert_err!(result);
}

pub async fn test_delete_dht_record_nonexistent(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let result = rc.delete_dht_record(*BOGUS_KEY).await;
    assert_err!(result);
}

pub async fn test_create_delete_dht_record_simple(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let rec = rc
        .create_dht_record(
            DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 1 }),
            Some(CRYPTO_KIND_VLD0),
        )
        .await
        .unwrap();

    let dht_key = *rec.key();
    rc.close_dht_record(dht_key).await.unwrap();
    rc.delete_dht_record(dht_key).await.unwrap();
}

pub async fn test_get_dht_value_nonexistent(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let rec = rc
        .create_dht_record(
            DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 1 }),
            Some(CRYPTO_KIND_VLD0),
        )
        .await
        .unwrap();
    let dht_key = *rec.key();
    let result = rc.get_dht_value(dht_key, 0, false).await;
    assert_eq!(result.expect("should not be error"), None);

    rc.close_dht_record(dht_key).await.unwrap();
    rc.delete_dht_record(dht_key).await.unwrap();
}

pub async fn test_set_get_dht_value(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let rec = rc
        .create_dht_record(
            DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 2 }),
            Some(CRYPTO_KIND_VLD0),
        )
        .await
        .unwrap();
    let dht_key = *rec.key();

    let test_value = String::from("BLAH BLAH BLAH").as_bytes().to_vec();
    // convert string to byte array
    let set_dht_value_result = rc.set_dht_value(dht_key, 0, test_value.clone()).await;
    assert_eq!(set_dht_value_result.expect("should be Ok(None)"), None);

    let get_dht_value_result_0_non_force = rc.get_dht_value(dht_key, 0, false).await;
    assert_eq!(
        get_dht_value_result_0_non_force
            .expect("should not be error")
            .expect("should hold a value")
            .data(),
        test_value.clone()
    );

    // works in python, fails in rust
    let get_dht_value_result_0_force = rc.get_dht_value(dht_key, 0, true).await;
    assert_eq!(
        get_dht_value_result_0_force
            .expect("should not be error")
            .expect("should hold a value")
            .data(),
        test_value.clone()
    );

    let get_dht_value_result_1_non_force = rc.get_dht_value(dht_key, 1, false).await;
    assert_eq!(
        get_dht_value_result_1_non_force.expect("should not be error"),
        None
    );

    // assert_eq!(
    //     get_dht_value_result_0_non_force.expect("should hold value"),
    //     get_dht_value_result_1_non_force.expect("should hold value")
    // );

    rc.close_dht_record(dht_key).await.unwrap();
    rc.delete_dht_record(dht_key).await.unwrap();
}

pub async fn test_open_writer_dht_value(api: VeilidAPI) {
    let rc = api
        .routing_context()
        .unwrap()
        .with_safety(SafetySelection::Unsafe(Sequencing::EnsureOrdered))
        .unwrap();

    let rec = rc
        .create_dht_record(
            DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 2 }),
            Some(CRYPTO_KIND_VLD0),
        )
        .await
        .unwrap();
    let key = *rec.key();
    let owner = rec.owner();
    let secret = rec.owner_secret().unwrap();
    let keypair = KeyPair::new(*owner, *secret);

    let test_value_1 = String::from("Qwertyuiop Asdfghjkl Zxcvbnm")
        .as_bytes()
        .to_vec();
    let test_data_2 = String::from("1234567890").as_bytes().to_vec();
    let test_data_3 = String::from("!@#$%^&*()").as_bytes().to_vec();

    // Scenario 1
    // 1. Write test data 1 to subkey 1,
    // 2. Read data from subkey 1, without force_refresh, check data, sequence and owner
    // 3. Read data from subkey 0, should return an error
    // 4. Write test data to subkey 0
    // 5. Read data from subkey 0 with force_refresh, check data
    // 6. Read data from subkey 1 with force_refresh, check data
    // 7. Overwrite value 1 twice, check that there's no errors
    let set_dht_test_value_1_result = rc.set_dht_value(key, 1, test_value_1.clone()).await;
    assert!(set_dht_test_value_1_result.is_ok());

    let get_dht_value_result_1_non_force = rc.get_dht_value(key, 1, false).await;
    assert!(get_dht_value_result_1_non_force.is_ok());
    let get_dht_value_result_1_non_force = get_dht_value_result_1_non_force
        .unwrap()
        .expect("should hold value");
    assert_eq!(get_dht_value_result_1_non_force.data(), test_value_1);
    assert_eq!(get_dht_value_result_1_non_force.seq(), 0);
    assert_eq!(get_dht_value_result_1_non_force.writer(), owner);

    let get_dht_value_result_0_non_force = rc.get_dht_value(key, 0, false).await;
    assert_eq!(
        get_dht_value_result_0_non_force.expect("should not be error"),
        None
    );

    let set_dht_test_value_0_result = rc.set_dht_value(key, 0, test_data_2.clone()).await;
    assert!(set_dht_test_value_0_result.is_ok());

    let get_dht_value_result_0_force = rc.get_dht_value(key, 0, true).await;
    assert_eq!(
        get_dht_value_result_0_force
            .expect("should be OK(result)")
            .expect("should hold value")
            .data(),
        test_data_2
    );

    let get_dht_value_result_1_force = rc.get_dht_value(key, 1, true).await;
    assert_eq!(
        get_dht_value_result_1_force
            .expect("should be OK(result)")
            .expect("should hold value")
            .data(),
        test_value_1
    );

    let overwrite_value_1_result_1 = rc.set_dht_value(key, 1, test_value_1.clone()).await;
    assert!(overwrite_value_1_result_1.is_ok());

    let overwrite_value_1_result_2 = rc.set_dht_value(key, 1, test_data_2.clone()).await;
    assert!(overwrite_value_1_result_2.is_ok());

    // Now that we initialized some subkeys
    // and verified they stored correctly
    // Delete things locally and reopen and see if we can write
    // with the same writer key

    rc.close_dht_record(key).await.unwrap();
    rc.delete_dht_record(key).await.unwrap();

    // Scenario 2
    // 1. Open DHT record with existing keys
    // 2. Check record key, owner, record secret and schema against original values
    // 3. Write test data 3 to subkey 1, without updating a value check that it still
    //    holds test data 2, but sequence has incremented, check owner
    // 4. Check that subkey 1 can be overwritten
    // 5. Read data from subkey 1 with force_refresh, check data

    let rec = rc.open_dht_record(key, Some(keypair)).await;
    assert!(rec.is_ok());
    let rec = rec.unwrap();
    assert_eq!(rec.key().value, key.value);
    assert_eq!(rec.key().kind, key.kind);
    assert_eq!(rec.owner(), owner);
    assert_eq!(rec.owner_secret().unwrap(), secret);
    assert!(matches!(
        rec.schema().clone(),
        DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 2 })
    ));

    //Verify subkey 1 can be set before it is get but newer is available online
    let set_dht_test_value_1_result = rc.set_dht_value(key, 1, test_data_3.clone()).await;
    assert!(set_dht_test_value_1_result.is_ok());
    let vdtemp = set_dht_test_value_1_result.unwrap().unwrap();
    assert_eq!(vdtemp.data(), test_data_2);
    assert_eq!(vdtemp.seq(), 1);
    assert_eq!(vdtemp.writer(), owner);

    // Verify subkey 1 can be set a second time and it updates because seq is newer
    let set_dht_test_value_1_result = rc.set_dht_value(key, 1, test_data_3.clone()).await;
    assert!(set_dht_test_value_1_result.is_ok());

    // Verify the network got the subkey update with a refresh check
    let get_dht_value_result_1_force = rc.get_dht_value(key, 1, true).await;
    assert!(get_dht_value_result_1_force.is_ok());
    let get_dht_value_result_1_force = get_dht_value_result_1_force
        .expect("should be OK(result)")
        .expect("should hold value");
    assert_eq!(get_dht_value_result_1_force.data(), test_data_3);
    assert_eq!(get_dht_value_result_1_force.seq(), 2);
    assert_eq!(get_dht_value_result_1_force.writer(), owner);

    // Delete things locally and reopen and see if we can write
    // with a different writer key (should fail)
    rc.close_dht_record(key).await.unwrap();
    rc.delete_dht_record(key).await.unwrap();

    // Scenario 3
    // 1. Open DHT record with new keypair
    // 2. Check record key, owner, record secret and schema against original values
    // 3. Try writing to subkey 1, expect error
    // 4. Try writing to subkey 0, expect error

    let cs = api.crypto().unwrap().get(key.kind).unwrap();
    assert!(cs.validate_keypair(owner, secret));
    let other_keypair = cs.generate_keypair();

    let rec = rc.open_dht_record(key, Some(other_keypair)).await;
    assert!(rec.is_ok());
    let rec = rec.unwrap();
    assert_eq!(rec.key().value, key.value);
    assert_eq!(rec.key().kind, key.kind);
    assert_eq!(rec.owner(), owner);
    assert_eq!(rec.owner_secret(), None);
    let schema = rec.schema().clone();
    assert!(matches!(
        schema,
        DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 2 })
    ));

    // Verify subkey 1 can NOT be set because we have the wrong writer
    let set_dht_test_value_0_result = rc.set_dht_value(key, 1, test_value_1.clone()).await;
    assert_err!(set_dht_test_value_0_result);

    // Verify subkey 0 can NOT be set because we have the wrong writer
    let set_dht_test_value_0_result = rc.set_dht_value(key, 0, test_value_1.clone()).await;
    assert_err!(set_dht_test_value_0_result);

    rc.close_dht_record(key).await.unwrap();
    rc.delete_dht_record(key).await.unwrap();
}

// Network-related code to make sure veilid node is connetected to other peers

async fn wait_for_public_internet_ready(api: &VeilidAPI) {
    info!("wait_for_public_internet_ready");
    loop {
        let state = api.get_state().await.unwrap();
        if state.attachment.public_internet_ready {
            break;
        }
        sleep(5000).await;
    }
    info!("wait_for_public_internet_ready, done");
}

pub async fn test_all() {
    let (update_callback, config_callback) = setup_veilid_core();
    let api = api_startup(update_callback, config_callback)
        .await
        .expect("startup failed");

    let _ = api.attach().await;
    wait_for_public_internet_ready(&api).await;

    test_get_dht_value_unopened(api.clone()).await;
    test_open_dht_record_nonexistent_no_writer(api.clone()).await;
    test_close_dht_record_nonexistent(api.clone()).await;
    test_delete_dht_record_nonexistent(api.clone()).await;
    test_get_dht_value_nonexistent(api.clone()).await;
    test_create_delete_dht_record_simple(api.clone()).await;
    test_set_get_dht_value(api.clone()).await;
    test_open_writer_dht_value(api.clone()).await;

    api.shutdown().await;
}

use super::fixtures::*;
use crate::*;
use range_set_blaze::*;

// dht_record_descriptors

pub async fn test_dhtrecorddescriptor() {
    let orig = DHTRecordDescriptor {
        key: fix_typedkey(),
        owner: fix_cryptokey(),
        owner_secret: Some(fix_cryptokey()),
        schema: DHTSchema::DFLT(DHTSchemaDFLT { o_cnt: 4321 }),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// value_data

pub async fn test_valuedata() {
    let orig = ValueData {
        seq: 42,
        data: b"Brent Spiner".to_vec(),
        writer: fix_cryptokey(),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// value_subkey_range_set

pub async fn test_valuesubkeyrangeset() {
    let orig = ValueSubkeyRangeSet {
        data: RangeSetBlaze::from_iter([20..=30]),
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

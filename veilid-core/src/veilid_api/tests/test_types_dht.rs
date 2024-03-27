use super::fixtures::*;
use crate::*;
use range_set_blaze::*;

// dht_record_descriptors

pub async fn test_dhtrecorddescriptor() {
    let orig = DHTRecordDescriptor::new(
        fix_typedkey(),
        fix_cryptokey(),
        Some(fix_cryptokey()),
        DHTSchema::dflt(4321).unwrap(),
    );
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// value_data

pub async fn test_valuedata() {
    let orig = ValueData::new_with_seq(42, b"Brent Spiner".to_vec(), fix_cryptokey());
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// value_subkey_range_set

pub async fn test_valuesubkeyrangeset() {
    let orig = ValueSubkeyRangeSet::new_with_data(RangeSetBlaze::from_iter([20..=30]));
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

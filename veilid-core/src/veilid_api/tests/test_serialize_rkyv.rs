use crate::*;

pub async fn test_simple_string() {
    let plain = "basic string".to_string();
    let serialized = b"basic string\x0c\x00\x00\x00\xf4\xff\xff\xff".to_vec();

    let a = to_rkyv(&plain);
    assert_eq!(a.unwrap(), serialized);

    let b = from_rkyv::<String>(serialized);
    assert_eq!(b.unwrap(), plain);
}

pub async fn test_all() {
    test_simple_string().await;
}

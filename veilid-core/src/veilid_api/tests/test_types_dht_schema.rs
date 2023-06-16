use super::fixtures::*;
use crate::*;

// dlft

pub async fn test_dhtschemadflt() {
    let orig = DHTSchemaDFLT { o_cnt: 9 };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// mod

pub async fn test_dhtschema() {
    let orig = DHTSchema::SMPL(DHTSchemaSMPL {
        o_cnt: 91,
        members: vec![
            DHTSchemaSMPLMember {
                m_key: fix_cryptokey(),
                m_cnt: 5,
            },
            DHTSchemaSMPLMember {
                m_key: fix_cryptokey(),
                m_cnt: 6,
            },
        ],
    });
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// smpl

pub async fn test_dhtschemasmplmember() {
    let orig = DHTSchemaSMPLMember {
        m_key: fix_cryptokey(),
        m_cnt: 7,
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub async fn test_dhtschemasmpl() {
    let orig = DHTSchemaSMPL {
        o_cnt: 91,
        members: vec![
            DHTSchemaSMPLMember {
                m_key: fix_cryptokey(),
                m_cnt: 8,
            },
            DHTSchemaSMPLMember {
                m_key: fix_cryptokey(),
                m_cnt: 9,
            },
        ],
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

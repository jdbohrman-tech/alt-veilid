use super::fixtures::*;
use crate::*;

// dlft

pub fn test_dhtschemadflt() {
    let orig = DHTSchemaDFLT::new(9);
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// mod

pub fn test_dhtschema() {
    let orig = DHTSchema::SMPL(
        DHTSchemaSMPL::new(
            91,
            vec![
                DHTSchemaSMPLMember {
                    m_key: fix_publickey(),
                    m_cnt: 5,
                },
                DHTSchemaSMPLMember {
                    m_key: fix_publickey(),
                    m_cnt: 6,
                },
            ],
        )
        .unwrap(),
    );
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

// smpl

pub fn test_dhtschemasmplmember() {
    let orig = DHTSchemaSMPLMember {
        m_key: fix_publickey(),
        m_cnt: 7,
    };
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

pub fn test_dhtschemasmpl() {
    let orig = DHTSchemaSMPL::new(
        91,
        vec![
            DHTSchemaSMPLMember {
                m_key: fix_publickey(),
                m_cnt: 8,
            },
            DHTSchemaSMPLMember {
                m_key: fix_publickey(),
                m_cnt: 9,
            },
        ],
    )
    .unwrap();
    let copy = deserialize_json(&serialize_json(&orig)).unwrap();

    assert_eq!(orig, copy);
}

use super::*;

// Don't trace these functions as they are used in the transfer of API logs, which will recurse!

// #[instrument(level = "trace", ret, err)]
pub fn deserialize_json<'a, T: de::Deserialize<'a> + Debug>(arg: &'a str) -> VeilidAPIResult<T> {
    serde_json::from_str(arg).map_err(|e| VeilidAPIError::ParseError {
        message: e.to_string(),
        value: format!(
            "deserialize_json:\n---\n{}\n---\n to type {}",
            arg,
            std::any::type_name::<T>()
        ),
    })
}
pub fn deserialize_json_bytes<'a, T: de::Deserialize<'a> + Debug>(
    arg: &'a [u8],
) -> VeilidAPIResult<T> {
    serde_json::from_slice(arg).map_err(|e| VeilidAPIError::ParseError {
        message: e.to_string(),
        value: format!(
            "deserialize_json_bytes:\n---\n{:?}\n---\n to type {}",
            arg,
            std::any::type_name::<T>()
        ),
    })
}

// #[instrument(level = "trace", ret, err)]
pub fn deserialize_opt_json<T: de::DeserializeOwned + Debug>(
    arg: Option<String>,
) -> VeilidAPIResult<T> {
    let arg = arg.as_ref().ok_or_else(|| VeilidAPIError::ParseError {
        message: "invalid null string".to_owned(),
        value: format!(
            "deserialize_json_opt: null to type {}",
            std::any::type_name::<T>()
        ),
    })?;
    deserialize_json(arg)
}

// #[instrument(level = "trace", ret, err)]
pub fn deserialize_opt_json_bytes<T: de::DeserializeOwned + Debug>(
    arg: Option<Vec<u8>>,
) -> VeilidAPIResult<T> {
    let arg = arg.as_ref().ok_or_else(|| VeilidAPIError::ParseError {
        message: "invalid null string".to_owned(),
        value: format!(
            "deserialize_json_opt: null to type {}",
            std::any::type_name::<T>()
        ),
    })?;
    deserialize_json_bytes(arg.as_slice())
}

// #[instrument(level = "trace", ret)]
pub fn serialize_json<T: Serialize + Debug>(val: T) -> String {
    match serde_json::to_string(&val) {
        Ok(v) => v,
        Err(e) => {
            panic!("failed to serialize json value: {}\nval={:?}", e, val);
        }
    }
}
pub fn serialize_json_bytes<T: Serialize + Debug>(val: T) -> Vec<u8> {
    match serde_json::to_vec(&val) {
        Ok(v) => v,
        Err(e) => {
            panic!(
                "failed to serialize json value to bytes: {}\nval={:?}",
                e, val
            );
        }
    }
}

pub mod as_human_base64 {
    use data_encoding::BASE64URL_NOPAD;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            let base64 = BASE64URL_NOPAD.encode(v);
            String::serialize(&base64, s)
        } else {
            Vec::<u8>::serialize(v, s)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        if d.is_human_readable() {
            let base64 = String::deserialize(d)?;
            BASE64URL_NOPAD
                .decode(base64.as_bytes())
                .map_err(serde::de::Error::custom)
        } else {
            Vec::<u8>::deserialize(d)
        }
    }
}

pub mod as_human_opt_base64 {
    use data_encoding::BASE64URL_NOPAD;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            let base64 = v.as_ref().map(|x| BASE64URL_NOPAD.encode(x));
            Option::<String>::serialize(&base64, s)
        } else {
            Option::<Vec<u8>>::serialize(v, s)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>, D::Error> {
        if d.is_human_readable() {
            let base64 = Option::<String>::deserialize(d)?;
            base64
                .map(|x| {
                    BASE64URL_NOPAD
                        .decode(x.as_bytes())
                        .map_err(serde::de::Error::custom)
                })
                .transpose()
        } else {
            Option::<Vec<u8>>::deserialize(d)
        }
    }
}

pub mod as_human_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, s: S) -> Result<S::Ok, S::Error>
    where
        T: Display + Serialize,
        S: Serializer,
    {
        if s.is_human_readable() {
            s.collect_str(value)
        } else {
            T::serialize(value, s)
        }
    }

    pub fn deserialize<'de, T, D>(d: D) -> Result<T, D::Error>
    where
        T: FromStr + Deserialize<'de>,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        if d.is_human_readable() {
            String::deserialize(d)?.parse().map_err(de::Error::custom)
        } else {
            T::deserialize(d)
        }
    }
}

pub mod as_human_opt_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, s: S) -> Result<S::Ok, S::Error>
    where
        T: Display + Serialize,
        S: Serializer,
    {
        if s.is_human_readable() {
            match value {
                Some(v) => s.collect_str(v),
                None => s.serialize_none(),
            }
        } else {
            Option::<T>::serialize(value, s)
        }
    }

    pub fn deserialize<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr + Deserialize<'de>,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        if d.is_human_readable() {
            match Option::<String>::deserialize(d)? {
                None => Ok(None),
                Some(v) => Ok(Some(v.parse::<T>().map_err(de::Error::custom)?)),
            }
        } else {
            Option::<T>::deserialize(d)
        }
    }
}

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

// #[instrument(level = "trace", ret)]
pub fn serialize_json<T: Serialize + Debug>(val: T) -> String {
    match serde_json::to_string(&val) {
        Ok(v) => v,
        Err(e) => {
            panic!("failed to serialize json value: {}\nval={:?}", e, val);
        }
    }
}

pub mod json_as_base64 {
    use data_encoding::BASE64URL_NOPAD;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = BASE64URL_NOPAD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        BASE64URL_NOPAD
            .decode(base64.as_bytes())
            .map_err(|e| serde::de::Error::custom(e))
    }
}

pub mod json_as_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod opt_json_as_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        match value {
            Some(v) => serializer.collect_str(v),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            None => Ok(None),
            Some(v) => Ok(Some(v.parse::<T>().map_err(de::Error::custom)?)),
        }
    }
}

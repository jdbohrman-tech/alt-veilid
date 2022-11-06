use super::*;
use crate::xx::*;
use crate::*;
use data_encoding::BASE64URL_NOPAD;
use js_sys::*;
use send_wrapper::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::*;
use web_sys::*;

#[derive(Clone)]
pub struct ProtectedStore {
    config: VeilidConfig,
}

impl ProtectedStore {
    pub fn new(config: VeilidConfig) -> Self {
        Self { config }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn delete_all(&self) -> EyreResult<()> {
        // Delete all known keys
        if self.remove_user_secret("node_id").await? {
            debug!("deleted protected_store key 'node_id'");
        }
        if self.remove_user_secret("node_id_secret").await? {
            debug!("deleted protected_store key 'node_id_secret'");
        }
        if self.remove_user_secret("_test_key").await? {
            debug!("deleted protected_store key '_test_key'");
        }
        if self.remove_user_secret("RouteSpecStore").await? {
            debug!("deleted protected_store key 'RouteSpecStore'");
        }
        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    pub async fn init(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn terminate(&self) {}

    fn keyring_name(&self) -> String {
        let c = self.config.get();
        if c.namespace.is_empty() {
            "veilid_protected_store".to_owned()
        } else {
            format!("veilid_protected_store_{}", c.namespace)
        }
    }

    fn browser_key_name(&self, key: &str) -> String {
        let c = self.config.get();
        if c.namespace.is_empty() {
            format!("__veilid_protected_store_{}", key)
        } else {
            format!("__veilid_protected_store_{}_{}", c.namespace, key)
        }
    }

    //#[instrument(level = "trace", skip(self, value), ret, err)]
    pub async fn save_user_secret_string(&self, key: &str, value: &str) -> EyreResult<bool> {
        if is_browser() {
            let win = match window() {
                Some(w) => w,
                None => {
                    bail!("failed to get window");
                }
            };

            let ls = match win
                .local_storage()
                .map_err(map_jsvalue_error)
                .wrap_err("exception getting local storage")?
            {
                Some(l) => l,
                None => {
                    bail!("failed to get local storage");
                }
            };

            let vkey = self.browser_key_name(key);

            let prev = match ls
                .get_item(&vkey)
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")?
            {
                Some(_) => true,
                None => false,
            };

            ls.set_item(&vkey, value)
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")?;

            Ok(prev)
        } else {
            unimplemented!()
        }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn load_user_secret_string(&self, key: &str) -> EyreResult<Option<String>> {
        if is_browser() {
            let win = match window() {
                Some(w) => w,
                None => {
                    bail!("failed to get window");
                }
            };

            let ls = match win
                .local_storage()
                .map_err(map_jsvalue_error)
                .wrap_err("exception getting local storage")?
            {
                Some(l) => l,
                None => {
                    bail!("failed to get local storage");
                }
            };

            let vkey = self.browser_key_name(key);

            ls.get_item(&vkey)
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")
        } else {
            unimplemented!();
        }
    }

    #[instrument(level = "trace", skip(self, value))]
    pub async fn save_user_secret_cbor<T>(&self, key: &str, value: &T) -> EyreResult<bool>
    where
        T: Serialize,
    {
        let v = serde_cbor::to_vec(value).wrap_err("couldn't store as CBOR")?;
        self.save_user_secret(&key, &v).await
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn load_user_secret_cbor<T>(&self, key: &str) -> EyreResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let out = self.load_user_secret(key).await?;
        let b = match out {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };

        let obj = serde_cbor::from_slice::<T>(&b).wrap_err("failed to deserialize")?;
        Ok(Some(obj))
    }

    #[instrument(level = "trace", skip(self, value), ret, err)]
    pub async fn save_user_secret(&self, key: &str, value: &[u8]) -> EyreResult<bool> {
        let mut s = BASE64URL_NOPAD.encode(value);
        s.push('!');

        self.save_user_secret_string(key, s.as_str()).await
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn load_user_secret(&self, key: &str) -> EyreResult<Option<Vec<u8>>> {
        let mut s = match self.load_user_secret_string(key).await? {
            Some(s) => s,
            None => {
                return Ok(None);
            }
        };

        if s.pop() != Some('!') {
            bail!("User secret is not a buffer");
        }

        let mut bytes = Vec::<u8>::new();
        let res = BASE64URL_NOPAD.decode_len(s.len());
        match res {
            Ok(l) => {
                bytes.resize(l, 0u8);
            }
            Err(_) => {
                bail!("Failed to decode");
            }
        }

        let res = BASE64URL_NOPAD.decode_mut(s.as_bytes(), &mut bytes);
        match res {
            Ok(_) => Ok(Some(bytes)),
            Err(_) => bail!("Failed to decode"),
        }
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn remove_user_secret(&self, key: &str) -> EyreResult<bool> {
        if is_browser() {
            let win = match window() {
                Some(w) => w,
                None => {
                    bail!("failed to get window");
                }
            };

            let ls = match win
                .local_storage()
                .map_err(map_jsvalue_error)
                .wrap_err("exception getting local storage")?
            {
                Some(l) => l,
                None => {
                    bail!("failed to get local storage");
                }
            };

            let vkey = self.browser_key_name(key);

            match ls
                .get_item(&vkey)
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")?
            {
                Some(_) => {
                    ls.delete(&vkey)
                        .map_err(map_jsvalue_error)
                        .wrap_err("exception_thrown")?;
                    Ok(true)
                }
                None => Ok(false),
            }
        } else {
            unimplemented!();
        }
    }
}

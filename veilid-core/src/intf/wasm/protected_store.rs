use super::*;
use data_encoding::BASE64URL_NOPAD;

use web_sys::*;

impl_veilid_log_facility!("pstore");

#[derive(Debug)]
#[must_use]
pub struct ProtectedStore {
    registry: VeilidComponentRegistry,
}

impl_veilid_component!(ProtectedStore);

impl ProtectedStore {
    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self { registry }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub fn delete_all(&self) -> EyreResult<()> {
        for kpsk in &KNOWN_PROTECTED_STORE_KEYS {
            if let Err(e) = self.remove_user_secret(kpsk) {
                error!("failed to delete '{}': {}", kpsk, e);
            } else {
                veilid_log!(self debug "deleted table '{}'", kpsk);
            }
        }
        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn post_init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    async fn pre_terminate_async(&self) {}

    #[instrument(level = "debug", skip(self))]
    async fn terminate_async(&self) {}

    fn browser_key_name(&self, key: &str) -> String {
        let config = self.config();
        let c = config.get();
        if c.namespace.is_empty() {
            format!("__veilid_protected_store_{}", key)
        } else {
            format!("__veilid_protected_store_{}_{}", c.namespace, key)
        }
    }

    #[instrument(level = "trace", skip(self, key, value))]
    pub fn save_user_secret_string<K: AsRef<str> + fmt::Debug, V: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
        value: V,
    ) -> EyreResult<bool> {
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

            let vkey = self.browser_key_name(key.as_ref());

            let prev = ls
                .get_item(&vkey)
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")?
                .is_some();

            ls.set_item(&vkey, value.as_ref())
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")?;

            Ok(prev)
        } else {
            unimplemented!()
        }
    }

    #[instrument(level = "trace", skip(self, key))]
    pub fn load_user_secret_string<K: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
    ) -> EyreResult<Option<String>> {
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

            let vkey = self.browser_key_name(key.as_ref());

            ls.get_item(&vkey)
                .map_err(map_jsvalue_error)
                .wrap_err("exception_thrown")
        } else {
            unimplemented!();
        }
    }

    #[instrument(level = "trace", skip(self, value))]
    pub fn save_user_secret_json<K, T>(&self, key: K, value: &T) -> EyreResult<bool>
    where
        K: AsRef<str> + fmt::Debug,
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value)?;
        self.save_user_secret(key, &v)
    }

    #[instrument(level = "trace", skip(self))]
    pub fn load_user_secret_json<K, T>(&self, key: K) -> EyreResult<Option<T>>
    where
        K: AsRef<str> + fmt::Debug,
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let out = self.load_user_secret(key)?;
        let b = match out {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };

        let obj = serde_json::from_slice(&b)?;
        Ok(Some(obj))
    }

    #[instrument(level = "trace", skip(self, value), ret, err)]
    pub fn save_user_secret<K: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
        value: &[u8],
    ) -> EyreResult<bool> {
        let mut s = BASE64URL_NOPAD.encode(value);
        s.push('!');

        self.save_user_secret_string(key, s.as_str())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub fn load_user_secret<K: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
    ) -> EyreResult<Option<Vec<u8>>> {
        let mut s = match self.load_user_secret_string(key)? {
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
    pub fn remove_user_secret<K: AsRef<str> + fmt::Debug>(&self, key: K) -> EyreResult<bool> {
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

            let vkey = self.browser_key_name(key.as_ref());

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

use super::*;
use data_encoding::BASE64URL_NOPAD;
use keyring_manager::*;
use std::path::Path;

impl_veilid_log_facility!("pstore");

pub struct ProtectedStoreInner {
    keyring_manager: Option<KeyringManager>,
}
impl fmt::Debug for ProtectedStoreInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProtectedStoreInner").finish()
    }
}

#[derive(Debug)]
#[must_use]
pub struct ProtectedStore {
    registry: VeilidComponentRegistry,
    inner: Mutex<ProtectedStoreInner>,
}

impl_veilid_component!(ProtectedStore);

impl ProtectedStore {
    fn new_inner() -> ProtectedStoreInner {
        ProtectedStoreInner {
            keyring_manager: None,
        }
    }

    pub(crate) fn new(registry: VeilidComponentRegistry) -> Self {
        Self {
            registry,
            inner: Mutex::new(Self::new_inner()),
        }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub fn delete_all(&self) -> EyreResult<()> {
        for kpsk in &KNOWN_PROTECTED_STORE_KEYS {
            if let Err(e) = self.remove_user_secret(kpsk) {
                veilid_log!(self error "failed to delete '{}': {}", kpsk, e);
            } else {
                veilid_log!(self debug "deleted protected store key '{}'", kpsk);
            }
        }
        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn init_async(&self) -> EyreResult<()> {
        let delete = {
            let config = self.config();
            let c = config.get();
            let mut inner = self.inner.lock();
            if !c.protected_store.always_use_insecure_storage {
                // Attempt to open the secure keyring
                cfg_if! {
                    if #[cfg(target_os = "android")] {
                        let maybe_km = KeyringManager::new_secure(&c.program_name, crate::intf::android::get_android_globals());
                    } else {
                        let maybe_km = KeyringManager::new_secure(&c.program_name);
                    }
                }

                inner.keyring_manager = match maybe_km {
                    Ok(v) => Some(v),
                    Err(e) => {
                        veilid_log!(self info "Secure key storage service unavailable, falling back to direct disk-based storage: {}", e);
                        None
                    }
                };
            }
            if (c.protected_store.always_use_insecure_storage
                || c.protected_store.allow_insecure_fallback)
                && inner.keyring_manager.is_none()
            {
                let directory = Path::new(&c.protected_store.directory);
                let insecure_keyring_file = directory.to_owned().join(format!(
                    "insecure_keyring{}",
                    if c.namespace.is_empty() {
                        "".to_owned()
                    } else {
                        format!("_{}", c.namespace)
                    }
                ));

                // Ensure permissions are correct
                ensure_file_private_owner(&insecure_keyring_file).map_err(|e| eyre!("{}", e))?;

                // Open the insecure keyring
                inner.keyring_manager = Some(
                    KeyringManager::new_insecure(&c.program_name, &insecure_keyring_file)
                        .wrap_err("failed to create insecure keyring")?,
                );
            }
            if inner.keyring_manager.is_none() {
                bail!("Could not initialize the protected store.");
            }
            c.protected_store.delete
        };

        if delete {
            self.delete_all()?;
        }

        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn post_init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    async fn pre_terminate_async(&self) {}

    #[instrument(level = "debug", skip(self))]
    async fn terminate_async(&self) {
        *self.inner.lock() = Self::new_inner();
    }

    fn service_name(&self) -> String {
        let config = self.config();
        let c = config.get();
        if c.namespace.is_empty() {
            "veilid_protected_store".to_owned()
        } else {
            format!("veilid_protected_store_{}", c.namespace)
        }
    }

    #[instrument(level = "trace", skip(self, value), ret, err)]
    pub fn save_user_secret_string<K: AsRef<str> + fmt::Debug, V: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
        value: V,
    ) -> EyreResult<bool> {
        let inner = self.inner.lock();
        inner
            .keyring_manager
            .as_ref()
            .ok_or_else(|| eyre!("Protected store not initialized"))?
            .with_keyring(&self.service_name(), key.as_ref(), |kr| {
                let existed = kr.get_value().is_ok();
                kr.set_value(value.as_ref())?;
                Ok(existed)
            })
            .wrap_err("failed to save user secret")
    }

    #[instrument(level = "trace", skip(self), err)]
    pub fn load_user_secret_string<K: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
    ) -> EyreResult<Option<String>> {
        let inner = self.inner.lock();
        match inner
            .keyring_manager
            .as_ref()
            .ok_or_else(|| eyre!("Protected store not initialized"))?
            .with_keyring(&self.service_name(), key.as_ref(), |kr| kr.get_value())
        {
            Ok(v) => Ok(Some(v)),
            Err(KeyringError::NoPasswordFound) => Ok(None),
            Err(e) => Err(eyre!("Failed to load user secret: {}", e)),
        }
    }

    #[instrument(level = "trace", skip(self, value))]
    pub fn save_user_secret_json<K, T>(&self, key: K, value: &T) -> EyreResult<bool>
    where
        K: AsRef<str> + fmt::Debug,
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value)?;
        self.save_user_secret(&key, &v)
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
        let inner = self.inner.lock();
        match inner
            .keyring_manager
            .as_ref()
            .ok_or_else(|| eyre!("Protected store not initialized"))?
            .with_keyring(&self.service_name(), key.as_ref(), |kr| kr.delete_value())
        {
            Ok(_) => Ok(true),
            Err(KeyringError::NoPasswordFound) => Ok(false),
            Err(e) => Err(eyre!("Failed to remove user secret: {}", e)),
        }
    }
}

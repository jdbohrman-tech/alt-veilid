use super::*;
use data_encoding::BASE64URL_NOPAD;
use keyring_manager::*;
use std::path::Path;

pub struct ProtectedStoreInner {
    keyring_manager: Option<KeyringManager>,
}

#[derive(Clone)]
pub struct ProtectedStore {
    config: VeilidConfig,
    inner: Arc<Mutex<ProtectedStoreInner>>,
}

impl ProtectedStore {
    fn new_inner() -> ProtectedStoreInner {
        ProtectedStoreInner {
            keyring_manager: None,
        }
    }

    pub fn new(config: VeilidConfig) -> Self {
        Self {
            config,
            inner: Arc::new(Mutex::new(Self::new_inner())),
        }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn delete_all(&self) -> EyreResult<()> {
        for kpsk in &KNOWN_PROTECTED_STORE_KEYS {
            if let Err(e) = self.remove_user_secret(kpsk).await {
                error!("failed to delete '{}': {}", kpsk, e);
            } else {
                debug!("deleted table '{}'", kpsk);
            }
        }
        Ok(())
    }

    #[instrument(level = "debug", skip(self), err)]
    pub async fn init(&self) -> EyreResult<()> {
        let delete = {
            let c = self.config.get();
            let mut inner = self.inner.lock();
            if !c.protected_store.always_use_insecure_storage {
                // Attempt to open the secure keyring
                cfg_if! {
                    if #[cfg(target_os = "android")] {
                        inner.keyring_manager = KeyringManager::new_secure(&c.program_name, crate::intf::android::get_android_globals()).ok();
                    } else {
                        inner.keyring_manager = KeyringManager::new_secure(&c.program_name).ok();
                    }
                }
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
            self.delete_all().await?;
        }

        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn terminate(&self) {
        *self.inner.lock() = Self::new_inner();
    }

    fn service_name(&self) -> String {
        let c = self.config.get();
        if c.namespace.is_empty() {
            "veilid_protected_store".to_owned()
        } else {
            format!("veilid_protected_store_{}", c.namespace)
        }
    }

    #[instrument(level = "trace", skip(self, value), ret, err)]
    pub async fn save_user_secret_string<K: AsRef<str> + fmt::Debug, V: AsRef<str> + fmt::Debug>(
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
    pub async fn load_user_secret_string<K: AsRef<str> + fmt::Debug>(
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
    pub async fn save_user_secret_json<K, T>(&self, key: K, value: &T) -> EyreResult<bool>
    where
        K: AsRef<str> + fmt::Debug,
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value)?;
        self.save_user_secret(&key, &v).await
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn load_user_secret_json<K, T>(&self, key: K) -> EyreResult<Option<T>>
    where
        K: AsRef<str> + fmt::Debug,
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let out = self.load_user_secret(key).await?;
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
    pub async fn save_user_secret<K: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
        value: &[u8],
    ) -> EyreResult<bool> {
        let mut s = BASE64URL_NOPAD.encode(value);
        s.push('!');

        self.save_user_secret_string(key, s.as_str()).await
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn load_user_secret<K: AsRef<str> + fmt::Debug>(
        &self,
        key: K,
    ) -> EyreResult<Option<Vec<u8>>> {
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
    pub async fn remove_user_secret<K: AsRef<str> + fmt::Debug>(&self, key: K) -> EyreResult<bool> {
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

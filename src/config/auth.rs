use derive_getters::Getters;
use once_cell::sync::OnceCell;
use serde::Deserialize;

static AUTH_CONFIG: OnceCell<Auth> = OnceCell::new();

pub fn set_auth_config(auth: Auth) {
    AUTH_CONFIG
        .set(auth)
        .expect("Auth config has already been set");
}

pub fn get_auth_config() -> Option<&'static Auth> {
    AUTH_CONFIG.get()
}

#[derive(Debug, Deserialize, Getters, Clone)]
pub struct Auth {
    pub(crate) username: String,
    pub(crate) password: String,
    #[serde(rename = "jwtSecret")]
    pub(crate) jwt_secret: String,
    #[serde(rename = "expireMinutes")]
    pub(crate) expire_minutes: usize,
}

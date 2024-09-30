use delegate::delegate;
use dotenv_config::EnvConfig;
use getset::{Getters, Setters};
use miette::{miette, Error, IntoDiagnostic, Result};
#[allow(unused_imports)]
use std::{str::FromStr, string::ToString};
// use serde::{Deserialize, Serialize};
// use tracing::warn;

#[allow(unused_imports)]
use crate::error::*;

#[derive(Debug, Clone, EnvConfig, Getters, Setters)]
pub struct Config {
    // pub keybindings: HashMap<String, String>, // FIXME: fails with new EnvConfig derive
    #[getset(get = "pub")]
    settings: Settings,
}

#[derive(Debug, Clone, EnvConfig, Getters, Setters)]
pub struct Settings {
    #[env_config(name = "GITLAB_BASE_URL", default = "https://gitlab.com")]
    #[getset(get = "pub", set = "pub")]
    pub base_url: String,
    #[env_config(name = "GITLAB_TOKEN", default = "")]
    #[getset(get = "pub", set = "pub")]
    pub token: String,
    #[env_config(name = "GITLAB_POLLING_INTERVAL_SEC", default = 30)]
    #[getset(get = "pub")]
    pub polling_interval: u32,
    #[env_config(name = "GITLAB_PROJECT", default = "")]
    #[getset(get = "pub", set = "pub")]
    pub project: String,
    #[env_config(name = "LOG_FILE", default = "")]
    #[getset(get = "pub", set = "pub")]
    pub log_file: String,
    #[env_config(name = "TICK_RATE", default = 1.0)]
    #[getset(get = "pub")]
    pub tick_rate: f64,
    #[env_config(name = "FRAME_RATE", default = 1.0)]
    #[getset(get = "pub")]
    pub frame_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config::init().map_err(|e| miette!(e)).unwrap()
    }
}

impl Config {
    delegate! {
        to self.settings {
            pub fn base_url(&self) -> &str;
            pub fn set_base_url(&mut self, val: String) -> &mut Settings;
            pub fn token(&self) -> &str;
            pub fn project(&self) -> &str;
            pub fn set_project(&mut self, val: String) -> &mut Settings;
            pub fn log_file(&self) -> &str;
            pub fn set_log_file(&mut self, v: String) -> &mut Settings;
            pub fn polling_interval(&self) -> &u32;
            pub fn tick_rate(&self) -> &f64;
            pub fn frame_rate(&self) -> &f64;
        }
    }

    #[inline]
    pub fn set_token(&mut self, val: String) -> Result<&mut Settings> {
        if Self::validate_token(&val) {
            self.settings.token = val;
            Ok(&mut self.settings)
        } else {
            Err(miette!("token is invalid, must be 46 characters long"))
        }
    }

    /// For example, check if the token is 46 characters long and contains only alphanumeric characters
    #[inline]
    fn validate_token(token: &str) -> bool {
        token.len() == 46 && token.chars().all(|c| c.is_alphanumeric())
    }
}

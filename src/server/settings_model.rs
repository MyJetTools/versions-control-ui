use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, EnvModel>,
    pub ssh_cert_prompt: bool,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }
}
#[derive(Serialize, Deserialize)]
pub struct EnvModel {
    pub url: String,
    pub host: String,
}

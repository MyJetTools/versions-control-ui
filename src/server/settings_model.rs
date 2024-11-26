use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, EnvModel>,
    pub ssh_pass_key_promt: Option<bool>,
    pub ssh_private_key_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EnvModel {
    pub url: String,
    pub host: String,
}

use std::collections::{BTreeMap, HashMap};

use my_ssh::SshCredentialsSettingsModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, EnvModel>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }
    pub async fn get_env_url(&self, env: &str) -> my_ssh::OverSshConnectionSettings {
        if let Some(result) = self.envs.get(env) {
            return my_ssh::OverSshConnectionSettings::parse(
                &result.url,
                self.ssh_credentials.as_ref(),
            )
            .await;
        }

        panic!("Can not get settings for env: '{}'", env);
    }
}
#[derive(Serialize, Deserialize)]
pub struct EnvModel {
    pub url: String,
    pub host: String,
}

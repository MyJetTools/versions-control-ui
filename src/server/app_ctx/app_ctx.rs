use std::sync::Arc;

use crate::server::settings_model::SettingsModel;
use flurl::FlUrl;
use my_settings_reader::SettingsReader;
use my_ssh::SshSessionsPool;

pub struct AppContext {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub ssh_sessions_pool: Arc<SshSessionsPool>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.versions-control-ui"),
            ssh_sessions_pool: SshSessionsPool::new().into(),
        }
    }

    pub async fn get_fl_url(&self, env_id: &str) -> (FlUrl, String) {
        let env = self
            .settings_reader
            .get(|settings| {
                settings
                    .envs
                    .get(env_id)
                    .map(|itm| (itm.url.clone(), itm.host.clone()))
            })
            .await;

        if env.is_none() {
            panic!("Can not find env with id '{}'", env_id);
        }

        let (url, host) = env.unwrap();

        (FlUrl::new(url), host)
    }
}

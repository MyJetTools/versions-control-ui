use crate::server::settings_model::SettingsModel;
use flurl::FlUrl;
use my_settings_reader::SettingsReader;
use tokio::sync::Mutex;

use super::{CertData, SshPrivateKeyCache};

pub struct AppContext {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub ssh_private_key: Mutex<Option<SshPrivateKeyCache>>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.versions-control-ui"),
            ssh_private_key: Mutex::new(None),
        }
    }

    pub async fn post_ssh_pass_phrase(&self, pass_phrase: Option<String>) {
        let mut ssh_private_key = self.ssh_private_key.lock().await;

        if let Some(ssh_private_key) = &mut *ssh_private_key {
            ssh_private_key.set_ssh_pass_phrase(pass_phrase);
            return;
        }

        let path = self
            .settings_reader
            .get(|settings| settings.ssh_private_key_path.clone())
            .await;
        *ssh_private_key = Some(SshPrivateKeyCache::new(path, pass_phrase).await);
    }

    async fn get_ssh_private_key(&self) -> Option<CertData> {
        let mut ssh_private_key = self.ssh_private_key.lock().await;

        loop {
            if let Some(ssh_private_key) = &*ssh_private_key {
                return ssh_private_key.get_cert_data();
            }

            let path = self
                .settings_reader
                .get(|settings| settings.ssh_private_key_path.clone())
                .await;
            *ssh_private_key = Some(SshPrivateKeyCache::new(path, None).await);
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

        let fl_url = FlUrl::new(url);

        if fl_url.via_ssh() {
            if let Some(cert_data) = self.get_ssh_private_key().await {
                return (
                    fl_url.set_ssh_private_key(cert_data.private_key, cert_data.pass_phrase),
                    host,
                );
            }
        }
        (fl_url, host)
    }
}

use crate::server::settings_model::SettingsModel;
use flurl::FlUrl;
use my_settings_reader::SettingsReader;
use tokio::sync::Mutex;

pub struct SshCertificate {
    pub private_key: String,
    pub pass_phrase: String,
}

pub struct AppContext {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub cert: Mutex<Option<SshCertificate>>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.versions-control-ui"),
            cert: Mutex::new(None),
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
            let ssh_key_access = self.cert.lock().await;

            if let Some(ssh_key) = ssh_key_access.as_ref() {
                let pass_phrase = if ssh_key.pass_phrase.is_empty() {
                    None
                } else {
                    Some(ssh_key.pass_phrase.clone())
                };

                return (
                    fl_url.set_ssh_private_key(ssh_key.private_key.clone(), pass_phrase),
                    host,
                );
            }
        }
        (fl_url, host)
    }
}

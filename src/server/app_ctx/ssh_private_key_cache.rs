#[derive(Debug, Clone)]
pub struct CertData {
    pub private_key: String,
    pub pass_phrase: Option<String>,
}

pub struct SshPrivateKeyCache {
    cert_data: Option<CertData>,
}

impl SshPrivateKeyCache {
    pub async fn new(path: Option<String>, pass_key: Option<String>) -> Self {
        if path.is_none() {
            return Self { cert_data: None };
        }

        let path = rust_extensions::file_utils::format_path(path.unwrap());

        let cert = tokio::fs::read_to_string(path.as_str()).await;

        if let Err(err) = &cert {
            panic!(
                "Can not read ssh private key file '{}'. Error: {}",
                path.as_str(),
                err
            );
        }

        Self {
            cert_data: CertData {
                private_key: cert.unwrap(),
                pass_phrase: pass_key,
            }
            .into(),
        }
    }

    pub fn get_cert_data(&self) -> Option<CertData> {
        self.cert_data.clone()
    }

    pub fn set_ssh_pass_phrase(&mut self, pass_phrase: Option<String>) {
        if let Some(cert_data) = &mut self.cert_data {
            cert_data.pass_phrase = pass_phrase;
        }
    }
}

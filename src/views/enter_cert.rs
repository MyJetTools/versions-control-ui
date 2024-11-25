use dioxus::prelude::*;

use crate::main_state::MainState;

#[component]
pub fn EnterCert() -> Element {
    let mut state = use_signal(|| EnterCertState::default());
    let state_read_access = state.read();

    if let Some(err) = state_read_access.err.as_ref() {
        return rsx! {
            div { class: "alert alert-danger", {err.as_str()} }
        };
    }

    let disabled = state_read_access.private_key.len() == 0;

    rsx! {

        div { style: "text-align: center; padding: 20px; width: 500px; margin: auto",
            div {
                label { "Enter passphrase" }
                input {
                    class: "form-control",
                    value: state_read_access.pass_phrase.as_str(),
                    r#type: "password",
                    oninput: move |e| {
                        state.write().pass_phrase = e.value();
                    }
                }
            }

            div {
                label { "Enter ssh cert private key" }

                textarea {
                    class: "form-control",
                    style: "min-height: 300px;",

                    value: state_read_access.private_key.as_str(),

                    oninput: move |e| {
                        state.write().private_key = e.value();
                    }
                }
            }

            button {
                class: "btn btn-primary",
                disabled,
                onclick: move |_| {
                    let cert_data = state.read().clone();
                    spawn(async move {
                        match post_cert(cert_data.private_key, cert_data.pass_phrase).await {
                            Ok(_) => {
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .prompt_ssh_cert = Some(false);
                            }
                            Err(err) => {
                                state.write().err = Some(err.to_string());
                            }
                        }
                    });
                },
                "Apply"
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct EnterCertState {
    pub private_key: String,
    pub pass_phrase: String,
    pub err: Option<String>,
}

#[server]
async fn post_cert(private_key: String, pass_phrase: String) -> Result<(), ServerFnError> {
    let ssh_cert = crate::server::SshCertificate {
        private_key,
        pass_phrase,
    };
    *crate::server::APP_CTX.cert.lock().await = Some(ssh_cert);
    Ok(())
}

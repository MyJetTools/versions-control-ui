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

            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    let cert_data = state.read().clone();
                    spawn(async move {
                        match post_cert(cert_data.pass_phrase).await {
                            Ok(_) => {
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .ssh_pass_key_promt = false;
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
    pub pass_phrase: String,
    pub err: Option<String>,
}

#[server]
async fn post_cert(pass_phrase: String) -> Result<(), ServerFnError> {
    let pass_phrase = if pass_phrase.is_empty() {
        None
    } else {
        Some(pass_phrase)
    };
    crate::server::APP_CTX
        .post_ssh_pass_phrase(pass_phrase)
        .await;
    Ok(())
}

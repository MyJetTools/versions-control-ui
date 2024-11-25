#![allow(non_snake_case)]

use crate::{
    dialogs::{DialogState, RenderDialog},
    main_state::MainState,
    views::*,
};

use dioxus::prelude::*;
use models::GetEnvsModel;

mod main_state;
#[cfg(feature = "server")]
mod server;

mod dialogs;
mod models;

mod storage;
mod views;

// let cfg = dioxus::fullstack::Config::new().addr(([0, 0, 0, 0], 8080));

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(App)
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    use_context_provider(|| Signal::new(DialogState::None));

    let mut main_state = consume_context::<Signal<MainState>>();

    let has_envs = { main_state.read().has_envs() };

    if has_envs {
        return rsx! {
            ActiveApp {}
        };
    }

    let resource = use_resource(|| get_envs());

    let data = resource.read_unchecked();

    match &*data {
        Some(data) => match data {
            Ok(result) => {
                main_state.write().set_environments(result);
                return rsx! {
                    ActiveApp {}
                };
            }
            Err(err) => {
                let err = format!("Error loading environments. Err: {}", err);
                return rsx! {
                    {err}
                };
            }
        },

        None => {
            return rsx! { "Loading environments..." };
        }
    }
}

#[component]
fn ActiveApp() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    if let Some(ssh_cert_prompt) = main_state_read_access.prompt_ssh_cert {
        if ssh_cert_prompt {
            return rsx! {
                EnterCert {}
            };
        }
    }

    rsx! {
        div { id: "top-panel", EnvsSelector {} }

        div { id: "main-panel", RenderDashboard {} }

        RenderDialog {}
    }
}

#[server]
pub async fn get_envs() -> Result<GetEnvsModel, ServerFnError> {
    let result = crate::server::APP_CTX.settings_reader.get_settings().await;

    Ok(GetEnvsModel {
        envs: result.envs.keys().cloned().collect(),
        ssh_cert_prompt: result.ssh_cert_prompt,
    })
}

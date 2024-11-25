use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;

use crate::{
    dialogs::{DialogState, EditTagDialogStateInner},
    main_state::MainState,
    models::AppVersionsHttpModel,
};

#[component]
pub fn RenderDashboard() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();
    let main_state_read_access = main_state.read();

    let env = main_state_read_access.selected_env.clone();

    let itm = match main_state_read_access.data.as_ref() {
        dioxus_utils::DataState::None => {
            spawn(async move {
                main_state.write().data = DataState::Loading;
                match get_to_release(env.to_string()).await {
                    Ok(result) => {
                        main_state.write().data = DataState::Loaded(result);
                    }
                    Err(err) => {
                        main_state.write().data = DataState::Error(format!("{}", err.to_string()));
                    }
                }
            });

            return rsx! {
                div {
                    h1 { "Loading..." }
                }
            };
        }
        dioxus_utils::DataState::Loading => {
            return rsx! {
                div {
                    h1 { "Loading..." }
                }
            };
        }
        dioxus_utils::DataState::Loaded(itm) => itm,
        dioxus_utils::DataState::Error(err) => {
            let err = format!("{}", err);
            return rsx! {
                div {
                    h1 { {err.as_str()} }
                }
            };
        }
    };

    let items = itm.vars.iter().map(|(key, value)| {
        let git_hub_repo_id = value
            .git_hub_repo_id
            .as_ref()
            .map(|itm| Rc::new(itm.to_string()));

        let (git_hub_version_to_render, color) =
            if let Some(git_hub_repo_id) = value.git_hub_repo_id.as_ref() {
                if let Some(git_hub_ver) = itm.git_hub_versions.get(git_hub_repo_id) {
                    let color = if git_hub_ver == value.ver.as_str() {
                        ""
                    } else {
                        "background: #ff00002b;"
                    };
                    (git_hub_ver.as_str(), color)
                } else {
                    ("", "background: #ff00002b;")
                }
            } else {
                ("", "background: #ff00002b;")
            };

        let git_hub_id = value.git_hub_repo_id.clone().unwrap_or_default();
        let env_id = env.clone();
        let ver = Rc::new(value.ver.clone());
        let tag = Rc::new(key.to_string());

        rsx! {
            tr { style: "{color}",
                td { {key.as_str()} }
                td { {value.ver.as_str()} }
                td { "{git_hub_id.as_str()}:{git_hub_version_to_render}" }
                td {
                    button {
                        class: "btn btn-primary btn-sm",
                        onclick: move |_| {
                            let version = ver.clone();
                            let tag = tag.clone();
                            let env_id = env_id.clone();
                            let git_hub_repo_id = git_hub_repo_id.clone();
                            consume_context::<Signal<DialogState>>()
                                .set(DialogState::EditToReleaseVersion {
                                    tag,
                                    version,
                                    git_hub_repo_id,
                                    on_ok: EventHandler::new(move |itm: EditTagDialogStateInner| {
                                        let env_id = env_id.clone();
                                        spawn(async move {
                                            let _ = save_to_release(
                                                    env_id.to_string(),
                                                    itm.tag,
                                                    itm.ver,
                                                    itm.git_hub_repo_id,
                                                )
                                                .await;
                                            main_state.write().data = DataState::None;
                                        });
                                    }),
                                })
                        },
                        "Update"
                    }
                }
            }
        }
    });

    rsx! {
        h4 { {itm.host.as_str()} }
        table { class: "table table-striped",
            thead {
                tr {
                    th { "Label" }
                    th { "Version" }
                    th { "GitHub" }
                    th {}
                }
            }
            tbody { {items} }
        }
    }
}

#[server]
async fn get_to_release(env_id: String) -> Result<AppVersionsHttpModel, ServerFnError> {
    println!("Loading for env: {}", env_id);
    let result = crate::server::requests::get_to_release(env_id.as_str()).await;
    Ok(result)
}

#[server]
async fn save_to_release(
    env_id: String,
    tag: String,
    ver: String,
    git_hub_repo_id: String,
) -> Result<(), ServerFnError> {
    let git_hub_repo_id = if git_hub_repo_id.is_empty() {
        None
    } else {
        Some(git_hub_repo_id)
    };
    let result =
        crate::server::requests::set_to_release(env_id.as_str(), tag, ver, git_hub_repo_id).await;
    Ok(result)
}

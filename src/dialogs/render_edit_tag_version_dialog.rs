use std::rc::Rc;

use dioxus::prelude::*;

use crate::dialogs::*;

#[component]
pub fn RenderEditTagVersionsDialog(
    tag: Rc<String>,
    ver: Rc<String>,
    git_hub_repo_id: Option<Rc<String>>,
    on_ok: EventHandler<EditTagDialogStateInner>,
) -> Element {
    let mut dialog_state: Signal<DialogState> = consume_context();
    let mut state = use_signal(|| EditTagDialogState::new(&tag, &ver, git_hub_repo_id.as_ref()));

    let state_read_access = state.read();

    let modal_body = rsx! {
        div { class: "modal-body",
            div { class: "input-group", style: "margin-top:10px",
                span { class: "input-group-text", "Tag" }
                input {
                    class: "form-control",
                    value: state_read_access.get_tag(),
                    oninput: move |e| {
                        state.write().set_tag(e.value().as_str());
                    }
                }
            }

            div { class: "input-group", style: "margin-top:10px",
                span { class: "input-group-text", "Version" }
                input {
                    class: "form-control",
                    value: state_read_access.get_ver(),
                    oninput: move |e| {
                        state.write().set_ver(e.value().as_str());
                    }
                }
            }

            div { class: "input-group", style: "margin-top:10px",
                span { class: "input-group-text", "git_hub_repo_id" }
                input {
                    class: "form-control",
                    value: state_read_access.get_git_hub_repo_id(),
                    oninput: move |e| {
                        state.write().set_git_hub_repo_id(e.value().as_str());
                    }
                }
            }
        }
    };

    rsx! {
        DialogTemplate {
            header: "Edit Tag Version".to_string(),
            dialog_state,
            modal_body,
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: !state_read_access.has_difference(),
                    onclick: move |_| {
                        let model = state.read().get_result();
                        on_ok.call(model);
                        dialog_state.set(DialogState::None);
                    },
                    "Save"
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct EditTagDialogStateInner {
    pub tag: String,
    pub ver: String,
    pub git_hub_repo_id: String,
}

pub struct EditTagDialogState {
    pub on_init: EditTagDialogStateInner,
    pub value: EditTagDialogStateInner,
}

impl EditTagDialogState {
    pub fn new(tag: &str, ver: &str, git_hub_repo_id: Option<&Rc<String>>) -> Self {
        let on_init = EditTagDialogStateInner {
            tag: tag.to_string(),
            ver: ver.to_string(),
            git_hub_repo_id: git_hub_repo_id
                .map(|itm| itm.to_string())
                .unwrap_or_default(),
        };
        Self {
            value: on_init.clone(),
            on_init,
        }
    }

    pub fn has_difference(&self) -> bool {
        self.on_init.tag != self.value.tag
            || self.on_init.ver != self.value.ver
            || self.on_init.git_hub_repo_id != self.value.git_hub_repo_id
    }

    pub fn get_tag(&self) -> &str {
        self.value.tag.as_str()
    }
    pub fn set_tag(&mut self, value: &str) {
        self.value.tag = value.trim().to_string();
    }

    pub fn get_ver(&self) -> &str {
        self.value.ver.as_str()
    }
    pub fn set_ver(&mut self, value: &str) {
        self.value.ver = value.trim().to_string();
    }

    pub fn get_git_hub_repo_id(&self) -> &str {
        self.value.git_hub_repo_id.as_str()
    }

    pub fn set_git_hub_repo_id(&mut self, value: &str) {
        self.value.git_hub_repo_id = value.trim().to_string();
    }

    pub fn get_result(&self) -> EditTagDialogStateInner {
        self.value.clone()
    }
}

use std::rc::Rc;

use dioxus::prelude::EventHandler;

use super::EditTagDialogStateInner;

#[derive(Clone)]
pub enum DialogState {
    None,
    EditToReleaseVersion {
        tag: Rc<String>,
        version: Rc<String>,
        git_hub_repo_id: Option<Rc<String>>,
        on_ok: EventHandler<EditTagDialogStateInner>,
    },
}

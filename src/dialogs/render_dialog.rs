use dioxus::prelude::*;

use super::*;

#[component]
pub fn RenderDialog() -> Element {
    let dialog_state = consume_context::<Signal<DialogState>>();

    let dialog = dialog_state.read().clone();
    match dialog {
        DialogState::None => {
            rsx! {
                div {}
            }
        }
        DialogState::EditToReleaseVersion {
            tag,
            version,
            git_hub_repo_id,
            on_ok,
        } => {
            rsx! {
                RenderEditTagVersionsDialog { tag, ver: version, git_hub_repo_id, on_ok }
            }
        }
    }
}

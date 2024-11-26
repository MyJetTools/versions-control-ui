use serde::*;

pub async fn set_to_release(
    env_id: &str,
    tag: String,
    version: String,
    git_hub_name: Option<String>,
) {
    let (fl_url, host) = crate::server::APP_CTX.get_fl_url(env_id).await;

    let model = SaveToReleaseVersion {
        tag,
        version,
        git_hub_name,
    };

    fl_url
        .with_header("host", host.as_str())
        .append_path_segment("api")
        .append_path_segment("Releases")
        .append_path_segment("ToReleaseVersion")
        .post_json(&model)
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveToReleaseVersion {
    tag: String,
    version: String,
    git_hub_name: Option<String>,
}

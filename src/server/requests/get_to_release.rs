use std::collections::BTreeMap;

use serde::*;

use crate::models::*;

pub async fn get_to_release(env_id: &str) -> AppVersionsHttpModel {
    let (fl_url, host) = crate::server::APP_CTX.get_fl_url(env_id).await;

    let mut response = fl_url
        .with_header("host", host.as_str())
        .append_path_segment("api")
        .append_path_segment("Releases")
        .append_path_segment("Settings")
        .get()
        .await
        .unwrap();

    let body = response.get_body_as_slice().await.unwrap();

    let result: ToReleaseHttpModel = serde_yaml::from_slice(body).unwrap();

    let (fl_url, host) = crate::server::APP_CTX.get_fl_url(env_id).await;
    let mut response = fl_url
        .with_header("host", host.as_str())
        .append_path_segment("api")
        .append_path_segment("GitHub")
        .get()
        .await
        .unwrap();

    let git_hub_versions: BTreeMap<String, String> =
        serde_json::from_slice(response.get_body_as_slice().await.unwrap()).unwrap();

    AppVersionsHttpModel {
        vars: result.vars,
        host,
        git_hub_versions,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToReleaseHttpModel {
    vars: BTreeMap<String, TagVersion>,
}

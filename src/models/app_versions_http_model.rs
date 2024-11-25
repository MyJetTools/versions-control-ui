use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppVersionsHttpModel {
    pub host: String,
    pub vars: BTreeMap<String, TagVersion>,
    pub git_hub_versions: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagVersion {
    pub ver: String,
    pub git_hub_repo_id: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEnvsModel {
    pub envs: Vec<String>,
    pub ssh_pass_key_promt: bool,
}

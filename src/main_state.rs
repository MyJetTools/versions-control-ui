use std::rc::Rc;

use dioxus_utils::DataState;

use crate::models::AppVersionsHttpModel;

pub struct MainState {
    pub selected_env: Rc<String>,
    pub envs: Option<Vec<Rc<String>>>,

    pub data: DataState<AppVersionsHttpModel>,
}

impl MainState {
    pub fn new() -> Self {
        let selected_env = crate::storage::selected_env::get();
        Self {
            selected_env: Rc::new(selected_env),
            envs: None,
            data: DataState::new(),
        }
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_environments(&mut self, envs: Vec<String>) {
        let envs: Vec<Rc<String>> = envs.into_iter().map(Rc::new).collect();

        if !envs
            .iter()
            .any(|itm| self.selected_env.as_str() == itm.as_str())
        {
            self.selected_env = envs.first().unwrap().clone();
        }

        self.envs = Some(envs);
    }

    pub fn set_active_env(&mut self, env: &str) {
        let found_value = self
            .envs
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|itm| itm.as_str() == env);

        if let Some(found_value) = found_value {
            crate::storage::selected_env::set(found_value.as_str());
            self.selected_env = found_value.clone();
        }

        self.data = DataState::new();
    }
}

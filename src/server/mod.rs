use app_ctx::AppContext;

mod app_ctx;
mod settings_model;

pub mod requests;

lazy_static::lazy_static! {
    pub static ref APP_CTX: AppContext = {
       AppContext::new()
    };
}
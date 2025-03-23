use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use server_functions::get_env_variable;

pub mod oauth;

pub mod app;

pub mod server_functions;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonalInfo {
    pub email: String,
    pub linkedin: String,
}

#[cfg(feature = "ssr")]
impl PersonalInfo {
    pub fn new() -> Self {
        let personal_email = get_env_variable("PERSONAL_EMAIL").expect("PERSONAL_EMAIL not set!");
        let linkedin = get_env_variable("LINKEDIN").expect("LINKEDIN not set!");
        PersonalInfo {
            email: personal_email,
            linkedin,
        }
    }
}

pub struct SmtpInfo {
    pub email: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub verified: bool,
    pub email: String,
}

#[cfg(feature = "ssr")]
impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        UserInfo {
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub success: bool,
    pub user_data: User,
    pub timestamp: i64,
}

#[cfg(feature = "ssr")]
impl SmtpInfo {
    pub fn new() -> Self {
        let smtp_email = get_env_variable("SMTP_EMAIL").expect("SMTP_EMAIL not set!");
        let smtp_key = get_env_variable("SMTP_KEY").expect("SMTP_KEY not set!");
        SmtpInfo {
            email: smtp_email,
            key: smtp_key,
        }
    }
}

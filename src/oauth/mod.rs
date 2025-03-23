pub mod oauth_client;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use serde::{Deserialize, Serialize};
        use crate::server_functions::encrypt_string;

        #[derive(Deserialize)]
        pub struct OauthResponse {
            pub code: String,
            pub state: String,
        }

        #[derive(Deserialize, Serialize)]
        pub struct TokenResponse{
            pub success: bool,
            pub access_token: String,
            pub refresh_token: String,
            pub username: String,
            pub expiry: i64
        }

        #[derive(Deserialize, Serialize)]
        pub struct SessionData {
            pub username: String,
            pub access_token: String,
            pub refresh_token: String,
            pub expiry: i64,
        }

        impl From<TokenResponse> for SessionData{
            fn from(value: TokenResponse) -> Self {
                SessionData { username: value.username, access_token: encrypt_string(&value.access_token).unwrap(), refresh_token: encrypt_string(&value.refresh_token).unwrap(), expiry: value.expiry }
            }
        }

    }
}

use cfg_if::cfg_if;
use leptos::{prelude::ServerFnError, server};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::server_functions::{generate_token, get_env_variable};

        lazy_static! {
            static ref OAUTH_URL: String = get_env_variable("OAUTH_URL").expect("OAUTH_URL is unset!");
            static ref CLIENT_ID: String = get_env_variable("CLIENT_ID").expect("CLIENT_ID is unset!");
            static ref CLIENT_SECRET: String = get_env_variable("CLIENT_SECRET").expect("CLIENT_SECRET IS UNSET!");
            static ref CLIENT: Client = reqwest::Client::builder().danger_accept_invalid_certs(true).danger_accept_invalid_hostnames(true).build().unwrap();
            static ref SITE_URL: String = get_env_variable("SITE_URL").expect("SITE_URL is not set!");
        }
        use actix_web::web;
        use lazy_static::lazy_static;
        use leptos_actix::extract;
        use redis::Client as RedisClient;
        use reqwest::Client;
        use redis::Commands;
        use std::time::{SystemTime, UNIX_EPOCH};
        use super::{OauthResponse, TokenResponse};
        use actix_web::HttpRequest;
        use crate::UserInfoResponse;
        use super::SessionData;
        use actix_identity::Identity;
        use actix_web::{HttpMessage, Responder};
    }
}

#[server(ProfileRedirect, "/api")]
pub async fn profile_redirect() -> Result<(), ServerFnError> {
    leptos_actix::redirect(format!("{}/user", OAUTH_URL.to_string()).as_str());

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let identity: Option<Identity> = extract().await?;
    Identity::logout(identity.expect("No user found in session!"));
    Ok(())
}

#[server(OauthRedirect, "/api")]
pub async fn oauth_redirect() -> Result<(), ServerFnError> {
    let user: Option<Identity> = extract().await?;
    if let Some(_) = user {
        leptos_actix::redirect(format!("{}/user", OAUTH_URL.to_string()).as_str());
        return Ok(());
    }
    let redis_client: web::Data<RedisClient> = extract().await?;

    let mut con = redis_client.get_connection().unwrap();

    let state = generate_token();

    let ttl_seconds = 300;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiry_time = now + ttl_seconds;

    () = con.zadd("states", &state, expiry_time).map_err(|err| {
        ServerFnError::new(format!("Error adding to redis cache!: {}", err.to_string()))
    })?;

    leptos_actix::redirect(
        format!(
            "{}/login?client_id={}&state={}",
            OAUTH_URL.to_string(),
            CLIENT_ID.to_string(),
            state
        )
        .as_str(),
    );

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn handle_oauth_response(
    request: HttpRequest,
    oauth_response: web::Query<OauthResponse>,
    redis_client: web::Data<RedisClient>,
) -> impl Responder {
    use actix_web::HttpResponse;

    use super::SessionData;

    let mut con = redis_client.get_connection().unwrap();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let OauthResponse { code, state } = oauth_response.into_inner();

    let redis_score: Option<u64> = con
        .zscore("states", &state)
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap();

    if redis_score.is_none_or(|expiry| expiry < now) {
        return HttpResponse::Unauthorized().body("OAuth credentials are invalid or expired");
    }

    () = con.zrem("states", state).unwrap();

    let params = [
        ("grant_type", "authorization_code"),
        ("authorization_code", &code),
    ];

    let token_response = request_access_token(&params)
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().body("Error querying authentication server")
        })
        .unwrap();

    let username = token_response.username.clone();

    let session_data = serde_json::to_string(&SessionData::from(token_response)).unwrap();

    () = con
        .set(username.clone(), session_data)
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap();

    Identity::login(&request.extensions(), username.clone())
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .unwrap();

    println!("Logged user in");

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[cfg(feature = "ssr")]
async fn request_access_token(params: &[(&str, &str)]) -> Result<TokenResponse, reqwest::Error> {
    let res = CLIENT
        .post(format!("{}/v0/oauth/token", OAUTH_URL.to_string()))
        .basic_auth(CLIENT_ID.to_string(), Some(CLIENT_SECRET.to_string()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    res.json::<TokenResponse>().await
}

#[cfg(feature = "ssr")]
pub async fn refresh_access_token(refresh_token: String) -> Result<TokenResponse, ServerFnError> {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &refresh_token),
    ];

    request_access_token(&params)
        .await
        .map_err(|err| ServerFnError::from(err))
}

#[cfg(feature = "ssr")]
pub async fn call_user_endpoint(
    session_data: SessionData,
) -> Result<UserInfoResponse, ServerFnError> {
    use crate::server_functions::decrypt_string;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let redis_client: web::Data<RedisClient> = extract().await?;

    let mut con = redis_client.get_connection().unwrap();

    let session_data = if session_data.expiry < now.try_into().unwrap() {
        // Todo: handle expired refresh_token
        let token_response =
            refresh_access_token(decrypt_string(&session_data.refresh_token).unwrap()).await?;

        let username = token_response.username.clone();

        let session_data = SessionData::from(token_response);

        let session_string = serde_json::to_string(&session_data)?;

        () = con.set(username.clone(), session_string)?;

        session_data
    } else {
        session_data
    };

    let res = CLIENT
        .get(format!(
            "{}/v0/users/info?username={}",
            OAUTH_URL.to_string(),
            &session_data.username
        ))
        .bearer_auth(decrypt_string(&session_data.access_token).unwrap())
        .send()
        .await?;

    res.json::<UserInfoResponse>()
        .await
        .map_err(|err| ServerFnError::from(err))
}

use std::sync::Arc;

use cfg_if::cfg_if;
use leptos::{prelude::ServerFnError, server};

use crate::{PersonalInfo, ResumeCache, UserInfo};

// Backend dependencies and functions
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use lettre::{
            message::{header, MultiPart, SinglePart},
            transport::smtp::authentication::Credentials,
            Message, SmtpTransport, Transport,
        };
        use dotenvy::dotenv;
        use std::env;
        use actix_web::HttpRequest;
        use maud::html;
        use actix_web::web;
        use leptos_actix::extract;
        use redis::Client;
        use redis::Commands;
        use aes_gcm::{
            Aes256Gcm, Key, Nonce,
            aead::{Aead, AeadCore, KeyInit, OsRng},
        };
        use actix_identity::Identity;
        use std::time::{SystemTime, UNIX_EPOCH};
        use crate::oauth::{oauth_client::call_user_endpoint, SessionData};
        use crate::SmtpInfo;

        use lazy_static::lazy_static;
        lazy_static!{
            static ref ENCRYPTION_KEY: String = get_env_variable("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set!");
        }


        use actix_web::Result;

        pub async fn get_request_data() -> Result<HttpRequest, ServerFnError> {
            use leptos_actix::extract;
            let req: actix_web::HttpRequest = extract()
                .await?;
            Ok(req)
        }

        pub fn get_env_variable(variable: &str) -> Option<String> {
            match std::env::var(variable) {
                Ok(env_variable) => Some(env_variable.trim().to_string()),
                Err(_) => {
                    dotenv().ok();

                    match env::var(variable) {
                        Ok(var_from_file) => Some(var_from_file.trim().to_string()),
                        Err(_) => None,
                    }
                }
            }
        }

        pub fn generate_token() -> String {
            use rand::distr::Alphanumeric;
            use rand::{Rng};

            let mut rng = rand::rng();

            let generated_token: String = (&mut rng)
                .sample_iter(Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();

            generated_token
        }

        pub fn encrypt_string(
            data: &String,
        ) -> Result<String, aes_gcm::Error> {
            let key = Key::<Aes256Gcm>::from_slice(&ENCRYPTION_KEY.as_bytes());

            let cipher = Aes256Gcm::new(&key);
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;

            let mut encrypted_data: Vec<u8> = nonce.to_vec();
            encrypted_data.extend_from_slice(&ciphertext);

            let output = hex::encode(encrypted_data);
            Ok(output)
        }

        pub fn decrypt_string(
            encrypted: &String,
        ) -> Result<String, aes_gcm::Error> {
            let encrypted_data = hex::decode(encrypted).expect("failed to decode hex string into vec");

            let key = Key::<Aes256Gcm>::from_slice(ENCRYPTION_KEY.as_bytes());

            // 12 digit nonce is prepended to encrypted data
            let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
            let nonce = Nonce::from_slice(nonce_arr);

            let cipher = Aes256Gcm::new(key);

            let plaintext = cipher
                .decrypt(nonce, ciphered_data)
                .expect("failed to decrypt data");

            Ok(String::from_utf8(plaintext).expect("failed to convert vector of bytes to string"))
        }

        pub fn generate_message_email(first_name: &String, last_name:  &String, email: &String, message: &String) -> String {
            html! {
                head {
                    title {"Welcome!"}
                    style type="text/css" {
                        "body {
                            font-family: Arial, sans-serif;
                            margin: 0;
                            padding: 0;
                            background-color: #f4f4f4;
                        }
                        .container {
                            max-width: 600px;
                            margin: 0 auto;
                            padding: 20px;
                            background-color: #fff;
                            border-radius: 8px;
                            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                        }
                        h1 {
                            color: #333;
                        }
                        p {
                            margin-bottom: 20px;
                            color: #666;
                        }
                        .btn {
                            display: inline-block;
                            padding: 10px 20px;
                            background-color: #007bff;
                            color: #FEFEFE;
                            text-decoration: none;
                            border-radius: 4px;
                        }
                        .btn:hover {
                            background-color: #0056b3;
                        }"
                    }
                }
                body{
                    div class="container" {
                        h1 {"New message from " (first_name)" "(last_name)}
                        p{(message)}
                        p{"Reply email: " (email)}
                    }
                }
            }
            .into_string()
        }

    }
}

#[server(SendEmail, "/api")]
pub async fn send_email(
    first_name: String,
    last_name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    let personal_info: web::Data<PersonalInfo> = extract().await?;
    let smtp_info: web::Data<SmtpInfo> = extract().await?;
    let email_body = generate_message_email(&first_name, &last_name, &email, &message);
    let generated_email = Message::builder()
        .from(
            format!("{first_name} {last_name} <{email}>")
                .parse()
                .unwrap(),
        )
        .reply_to(
            format!("{first_name} {last_name} <{email}>")
                .parse()
                .unwrap(),
        )
        .to(format!("Chris Bratti <{}>", &personal_info.email)
            .parse()
            .unwrap())
        .subject(format!("New message from {first_name} {last_name}"))
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(format!("New message from {first_name} {last_name}\n{message}\nReply email: {email}",
                        )),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(email_body),
                ),
        )
        .expect("failed to build email");

    let creds = Credentials::new(smtp_info.email.clone(), smtp_info.key.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let result = mailer.send(&generated_email);

    if result.is_err() {
        Err(ServerFnError::new(
            "There was an error sending your message! Please contact me via email",
        ))
    } else {
        Ok(())
    }
}

#[server]
pub async fn get_info() -> Result<PersonalInfo, ServerFnError> {
    let data: web::Data<PersonalInfo> = extract().await?;

    Ok(PersonalInfo {
        email: data.email.clone(),
        linkedin: data.linkedin.clone(),
    })
}

#[server]
pub async fn generate_pdf_link() -> Result<String, ServerFnError> {
    let redis_client: web::Data<Client> = extract().await?;

    let mut con = redis_client.get_connection().unwrap();
    let uuid = generate_token();

    let ttl_seconds = 300;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiry_time = now + ttl_seconds;

    () = con.zadd("pdf_links", &uuid, expiry_time).map_err(|err| {
        ServerFnError::new(format!("Error adding to redis cache!: {}", err.to_string()))
    })?;

    let url = format!("/{}/resume.pdf", uuid);
    Ok(url)
}

#[server]
pub async fn get_user_info() -> Result<Option<UserInfo>, ServerFnError> {
    println!("Fetching user session");
    let redis_client: web::Data<Client> = extract().await?;

    let mut con = redis_client.get_connection().unwrap();

    let user: Option<Identity> = extract().await?;

    if let Some(user) = user {
        let session_str: String = con.get(user.id().unwrap())?;
        let session_data: SessionData = serde_json::from_str(&session_str).unwrap();

        let user_info_response = call_user_endpoint(session_data).await?;

        Ok(Some(UserInfo::from(user_info_response.user_data)))
    } else {
        println!("No user found");
        Ok(None)
    }
}

#[server]
pub async fn get_resume_info() -> Result<Arc<ResumeCache>, ServerFnError> {
    let resume: web::Data<ResumeCache> = extract().await?;

    Ok(resume.into_inner())
}

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::NamedFile;
        use actix_web::{Result};
        use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
        use std::path::PathBuf;
        use actix_web::web;
        use redis::{Client, Commands};
        use chrisbratti_website::server_functions::get_env_variable;

        use lazy_static::lazy_static;

        lazy_static! {
            static ref RESUME_FILE_NAME: String = get_env_variable("RESUME_FILE_NAME").expect("RESUME_FILE_NAME not set!");
        }
    }
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::{sync::RwLock, time::Duration};

    use actix_files::Files;
    use actix_identity::IdentityMiddleware;
    use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
    use actix_web::{cookie::Key, *};
    use chrisbratti_website::{
        app::*,
        middleware::VerifyApiKey,
        oauth::oauth_client::handle_oauth_response,
        routes::resume_routes::{approve_pending_resume, upload_resume},
        server_functions::get_env_variable,
        services::resume_parsing_service::load_resume,
        PersonalInfo, ResumeCache, SmtpInfo,
    };
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let redis_connection_string =
        get_env_variable("REDIS_CONNECTION_STRING").expect("Connection string not set!");

    let resume = load_resume().await.unwrap();

    let resume_cache = web::Data::new(ResumeCache {
        resume: RwLock::new(resume),
    });

    let personal_info = web::Data::new(PersonalInfo::new());

    let smtp_info = web::Data::new(SmtpInfo::new());

    let redis_client =
        web::Data::new(redis::Client::open(redis_connection_string.clone()).unwrap());

    let secret_key = Key::from(
        get_env_variable("REDIS_KEY")
            .expect("REDIS_KEY not set!")
            .as_bytes(),
    );

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .service(download_pdf)
            .route("/auth", web::get().to(handle_oauth_response))
            .service(
                web::scope("/internal")
                    .wrap(VerifyApiKey)
                    .service(upload_resume)
                    .service(approve_pending_resume),
            )
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(resume_cache.clone())
            .app_data(personal_info.clone())
            .app_data(smtp_info.clone())
            .app_data(redis_client.clone())
            .wrap(
                IdentityMiddleware::builder()
                    .login_deadline(Some(Duration::new(259200, 0)))
                    .build(),
            )
            // Uses Session middleware for all Session info, uses Redis as a backend
            .wrap(
                SessionMiddleware::builder(store.clone(), secret_key.clone())
                    .cookie_secure(true)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::weeks(2)),
                    )
                    .build(),
            )
        //.wrap(middleware::Compress::default())
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("/{uuid}/resume.pdf")]
pub async fn download_pdf(
    path: web::Path<String>,
    redis_client: web::Data<Client>,
) -> Result<NamedFile, actix_web::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let uuid = path.into_inner();
    println!("Serving file for UUID: {}", uuid);

    let mut con = redis_client
        .get_connection()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not connect to redis!"))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let score: Option<u64> = con
        .zscore("pdf_links", &uuid)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error fetching from redis!"))?;

    if score.is_none_or(|expiry| expiry < now) {
        return Err(actix_web::error::ErrorInternalServerError(
            "Link invalid or expired",
        ));
    }

    // Update to resume path var
    let path: PathBuf = format!("uploads/{}.pdf", RESUME_FILE_NAME.as_str()).into();

    let file = NamedFile::open(path)?.set_content_disposition(ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(
            "ChrisBratti_Resume.pdf".to_string(),
        )],
    });

    Ok(file)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use chrisbratti_website::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

#![allow(non_snake_case)]

use std::sync::Arc;

use crate::oauth::oauth_client::*;
use crate::{server_functions::*, Resume};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/chrisbratti-website.css" />

        <Stylesheet
            id="googleicons"
            href="https://fonts.googleapis.com/icon?family=Material+Icons"
        />

        <Title text="Chris Bratti" />

        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/*any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let resume_result = Resource::new_blocking(|| (), |_| get_resume_info());
    view! {
        <div class="home-page">
            <div class="parallax">
                <div class="blurred-text">
                    <h1 class="extra-large">
                        "Hi, I'm "<span class="title-accent">"Chris Bratti."</span>
                    </h1>
                    <h4 class="subtitle" style="letter-spacing: 4px">"${software_engineer}"</h4>
                </div>
                <div class="down-arrow">"⇓"</div>
            </div>
            <div class="blurred-backdrop">
                <Suspense fallback= || view! {<p>"Loading..."</p>}>
                    {move || Suspend::new(async move {
                        let resume = resume_result.await.unwrap();
                        view! {
                            <AboutContainer resume=resume />
                        }
                    })}
                </Suspense>

            </div>
        </div>
    }
}

#[component]
fn DetailContainer(
    title: &'static str,
    children: Children,
    #[prop(optional)] open: bool,
) -> impl IntoView {
    view! {
        <details class="expandable-card" open=open>
            <summary>{title}</summary>
            <div class="card-content">
                <div class="sub-card-container" style="margin: 0px; padding: 5px">
                    {children()}
                </div>
            </div>
        </details>
    }
}

#[component]
fn AboutContainer(resume: Arc<Resume>) -> impl IntoView {
    let logout = ServerAction::<Logout>::new();
    let get_user = Resource::new_blocking(move || (logout.version().get()), |_| get_user_info());
    let oauth_redirect = ServerAction::<OauthRedirect>::new();
    let profile_redirect = ServerAction::<ProfileRedirect>::new();
    view! {
        <div class="main-container">
            <div class="card">
                <ul class="nav">
                    <li class="dropdown">
                        <Suspense fallback=|| (view! {<a class="dropbtn">"Login"</a>})>
                            {move || Suspend::new(async move {
                                let user_info = RwSignal::new(get_user.await.unwrap());
                                if user_info.get().is_some(){
                                    view! {
                                        <a class="dropbtn">{user_info.get().unwrap().first_name}</a>
                                        <ul class="dropdown-content">
                                            <li><a on:click={move |_| {profile_redirect.dispatch(ProfileRedirect {});}}>"Profile"</a></li>
                                            <li><a on:click={move |_| {logout.dispatch(Logout {}); }}>"Sign out"</a></li>
                                        </ul>
                                    }.into_any()
                                }else{
                                    view! {
                                        <a on:click=move |_| {
                                            oauth_redirect.dispatch(OauthRedirect {});
                                        } class="dropbtn">"Login"</a>
                                    }.into_any()
                                }
                            })}
                        </Suspense>
                    </li>
                    <div class="card-title">"About Me"</div>
                </ul>
                <DetailContainer title="Hi there!" open=true>
                    <div class="card-container">
                        <div class="experience-card" style="text-align:center">
                            <h4>
                                {"My name is Chris. I am a highly motivated Software Engineer with 6 years of professional experience designing, implementing, and deploying microservices.
                                I bring extensive experience developing RESTful APIs, test automation, and managing deployment pipelines.
                                I'm known for clear communication, a proven track record of delivering results, and a passion for solving complex problems.
                                "}
                            </h4>
                        </div>
                    </div>
                </DetailContainer>
                <DetailContainer title="Experience">
                    <ExperienceDetails resume=resume.clone() />
                </DetailContainer>

                <DetailContainer title="Skills">
                    <SkillsDetails />
                </DetailContainer>

                <DetailContainer title="Projects">
                    <ProjectsDetails />
                </DetailContainer>

                <DetailContainer title="Contact Me" open=true>
                    <ContactDetails />
                </DetailContainer>
            </div>
        </div>
    }.into_any()
}

#[component]
fn ContactForm() -> impl IntoView {
    let send_email = ServerAction::<SendEmail>::new();
    let pending = send_email.pending();
    let send_result = send_email.value();
    let spinner_class = RwSignal::new(String::from("circle-loader"));
    let check_class = RwSignal::new(String::from(""));
    let message = RwSignal::new(String::from("Sending message..."));
    view! {
        {move || {
            if send_result.get().is_some() || pending() {
                view! {
                    <div style="text-align: center">
                        <div class=move || spinner_class.get()>
                            <div class=move || check_class.get()></div>
                        </div>
                        <h3>{move || message.get()}</h3>
                    </div>
                    {move || {
                        if send_result.get().is_some() {
                            if send_result.get().unwrap().is_ok() {
                                spinner_class
                                    .set("circle-loader load-complete load-success".to_string());
                                check_class.set("checkmark draw".to_string());
                                message.set("Message sent! I'll get back to you soon!".to_string());
                            } else {
                                spinner_class
                                    .set("circle-loader load-complete load-failure".to_string());
                                check_class.set("".to_string());
                                message
                                    .set(
                                        "Message failed to send! Please contact me via email"
                                            .to_string(),
                                    );
                            }
                        }
                    }}
                }
                    .into_any()
            } else {
                view! {
                    <div class="experience-card-title">"Send me an email!"</div>
                    <ActionForm attr:class="action-form" action=send_email>
                        <label class="form-label">
                            "First Name"
                            <input
                                type="text"
                                name="first_name"
                                required=true
                                placeholder="First name"
                            />
                        </label>
                        <label class="form-label">
                            "Last Name"
                            <input
                                type="text"
                                name="last_name"
                                required=true
                                placeholder="Last name"
                            />
                        </label>
                        <label class="form-label">
                            "Email"
                            <input
                                type="email"
                                name="email"
                                required=true
                                placeholder="example@example.com"
                            />
                        </label>
                        <label class="form-label">
                            "Message"
                            <textarea name="message" required=true placeholder="Let's connect..." />
                        </label>
                        <input type="submit" disabled=move || { pending() } value="Send" />
                    </ActionForm>
                }
                    .into_any()
            }
        }}
    }
}

#[component]
fn ExperienceDetails(resume: Arc<Resume>) -> impl IntoView {
    let pdf_link = Resource::new_blocking(|| (), |_| generate_pdf_link());

    let experience_items = resume
        .experience
        .iter()
        .map(|experience_item| {
            let company = experience_item.company.as_ref().unwrap().clone();
            let title = experience_item.title.as_ref().unwrap().clone();
            let duration = experience_item.duration.as_ref().unwrap().clone();
            let desc = experience_item.desc.as_ref().unwrap().clone();
            view! {
                <div class="experience-card">
                <div class="experience-card-title">{company}</div>
                <h4>
                    <span class="text-italic">{format!("{} [{}]", title, duration)}</span>
                </h4>
                <ul class="modern-list">
                    {desc.into_iter().map(|item| view! { <li>{item}</li> }).collect_view()}
                </ul>
            </div>
            }
        })
        .collect_view();
    view! {
        {experience_items}
        <div class="experience-card" style="text-align: center">
            {move || Suspend::new(async move {
                let redirect_link = pdf_link.await.unwrap();
                view! {
                    <a class="btn" rel="external" href=redirect_link>
                        "View my full resume!"
                    </a>
                }
            })}
        </div>
    }
}

#[component]
fn ModernList(items: Vec<&'static str>) -> impl IntoView {
    view! {
        <ul class="modern-list">
            {items.into_iter().map(|item| view! { <li>{item}</li> }).collect_view()}
        </ul>
    }
}

#[component]
fn SkillComponent(title: &'static str, skills: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="experience-card">
            <div class="experience-card-title">{title}</div>
            <ul class="modern-list">
                <ModernList items=skills />
            </ul>
        </div>
    }
}

#[component]
fn SkillsDetails() -> impl IntoView {
    view! {
        <SkillComponent title="Languages" skills=vec!["Java", "Rust", "Bash", "Python", "SQL"] />
        <SkillComponent
            title="Tools and Frameworks"
            skills=vec![
                "Spring Boot",
                "jUnit",
                "AssertJ",
                "Mockito",
                "Leptos",
                "Actix Web",
                "Helm",
                "OAuth",
            ]
        />
        <SkillComponent
            title="DevOps and Deployment"
            skills=vec!["Kubernetes", "Docker", "AWS", "Jenkins", "ArgoCD", "Hashicorp"]
        />
        <SkillComponent
            title="Development Tools"
            skills=vec!["Git", "GitHub", "Swagger / OpenAPI", "Postman", "Bruno", "IntelliJ"]
        />
    }
}

#[component]
fn GitHubLink(project_name: &'static str, text: Option<&'static str>) -> impl IntoView {
    view! {
        <a
            class="btn github-link"
            target="_blank"
            rel="noopener noreferrer"
            href=format!("https://github.com/chris-bratti/{}", project_name)
        >
            {if text.is_none() { "View on GitHub!".to_string() } else { text.unwrap().to_string() }}
            <img src="/assets/github-mark.png" class="github-icon" />
        </a>
    }
}

#[component]
fn Project(
    title: &'static str,
    overview: &'static str,
    desc: &'static str,
    features: Vec<&'static str>,
    technologies: Vec<&'static str>,
    project_name: &'static str,
    link_text: Option<&'static str>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="experience-card">
            <div class="experience-card-title">{title}</div>
            <span class="text-italic">{overview}</span>
            <h5>{desc}</h5>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Features"</span>
            </h4>
            <ModernList items=features />
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Technologies/skills used"</span>
            </h4>
            <ModernList items=technologies />
            {if let Some(child_content) = children { child_content() } else { ().into_any() }}
            <GitHubLink project_name=project_name text=link_text />
        </div>
    }
}

#[component]
fn ProjectsDetails() -> impl IntoView {
    view! {
        <Project
            title="Auth-server"
            overview="A full stack Rust OAuth server written with the Leptos framework"
            desc="
            A fully features OAuth server written in Rust. The included docker-compose.yaml file makes it
            easy to deploy into a Docker or Kubernetes environment. Integrating new OAuth clients is simple and secure - clients can sign up using the REST endpoint and admins can
            approve or deny new clients in the Admin Dashboard.
            "
            features=vec![
                "Full OAuth Authorization Code flow - easy to integrate and protect applications",
                "Secure user signup, login, and password reset",
                "Persistent user session storage with actix_sessions",
                "Two factor authentication with Time-Based One Time Passwords (TOTP)",
                "Admin Dashboard",
                "Database hashing and encryption",
                "Redis integration for application caching",
            ]
            technologies=vec![
                "Rust, Leptos, Actix Web",
                "WASM, HTML, CSS",
                "Diesel, Postgres, Redis",
                "Argon2 hashing, database encryption",
                "Docker",
            ]
            project_name="auth-server"
            link_text=None
        >
            <a
                class="btn github-link"
                target="_blank"
                rel="noopener noreferrer"
                href="https://login.chrisbratti.com"
                style="margin-bottom: 15px"
            >
                "Check out a live demo of this project!"
            </a>
        </Project>

        <Project
            title="Resume Website"
            overview="Full stack resume and portfolio site (you're using it right now!)"
            desc="
            The repository for this site: a full-stack portfolio and resume website written in Rust. With a WASM-compiled front-end and an Actix-Web server, this site is 'blazingly fast'
            with minimal overhead.
            "
            features=vec![
                "Eye-catching UI designed with Leptos",
                "Fast and lightweight back-end written in Rust with Leptos and Actix",
                "Secure SMTP communication with TLS",
                "Protected download links",
            ]
            technologies=vec![
                "Rust, Leptos, Actix Web",
                "WASM, HTML, CSS",
                "Diesel, Postgres, Redis",
                "Docker",
            ]
            project_name="chrisbratti-website"
            link_text=None
        />

        <Project
            title="Wireguard-init"
            overview="Script to automate the creation of a WireGuard server and peers"
            desc="
            A lightweight and easy-to-use script to automate initializing, configuring, and deploying a WireGuard server and peers. I've developed and used this script to deploy several WireGuard servers to
            enable secure remote communication with my personal network when I'm away from home."
            features=vec![
                "Creates a secure peer-to-peer VPN network in just a few minutes",
                "Ability to customize IP address subnet, listening port, DNS server addresses, and more",
                "Automates peer/client setup",
                "Generates QR codes for configuring mobile peers",
                "Lightweight",
            ]
            technologies=vec!["Bash", "Networking", "Routing"]
            project_name="wireguard-init"
            link_text=None
        />
        <div class="experience-card" style="text-align: center">
            <GitHubLink project_name="" text=Some("Check out the rest of my projects on GitHub!") />
        </div>
    }
}

#[component]
fn ContactDetails() -> impl IntoView {
    let info_result = Resource::new_blocking(|| (), |_| get_info());
    view! {
        <div class="experience-card">
            <ContactForm />
        </div>
        <div class="experience-card">
            <div class="experience-card-title">"Or reach me here!"</div>
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    let info = info_result.await.unwrap();
                    view! {
                        <p>
                            <span class="custom-text-accent">
                                <i class="material-icons in-line-icon">mail</i>
                            </span>
                            {format!("{}", info.email)}
                        </p>
                        <p>
                            <span class="custom-text-accent">
                                <i class="material-icons in-line-icon">account_circle</i>
                            </span>
                            {format!("{}", info.linkedin)}
                        </p>
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <body class="home-page">
            <div class="blurred-backdrop">
                <div class="parallax">
                    <h1 class="extra-large">
                        <span class="custom-text-accent">"404 Not Found"</span>
                    </h1>
                    <h4 class="subtitle">
                        <br />
                        ":("
                    </h4>
                </div>
            </div>
        </body>
    }
}

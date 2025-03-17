#![allow(non_snake_case)]

use crate::server_functions::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/chrisbratti-website.css" />

        <Stylesheet
            id="googleicons"
            href="https://fonts.googleapis.com/icon?family=Material+Icons"
        />

        // sets the document title
        <Title text="Chris Bratti" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=TestHomePage />
                    <Route path=path!("/*any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DetailContainer(
    title: String,
    children: Children,
    #[prop(optional)] open: bool,
) -> impl IntoView {
    view! {
        <details class="expandable-card" open=open>
            <summary>{title}</summary>
            <div class="card-content">{children()}</div>
        </details>
    }
}

#[component]
fn AboutContainer() -> impl IntoView {
    view! {
        <div class="main-container">
            <div class="card-title">"About Me"</div>
            <div class="card">
                <DetailContainer title="Hi there!".to_owned() open=true>
                    <div class="sub-card-container">
                        <div class="card-container">
                            <div class="experience-card" style="text-align:center">
                                <h4>
                                    "My name is Chris. I am a highly motivated Software Engineer with 6 years of professional experience designing, implementing, and deploying microservices.
                                    I bring extensive experience developing RESTful APIs, test automation, and managing deployment pipelines.
                                    I'm known for clear communication, a proven track record of delivering results, and a passion for solving complex problems.
                                    "
                                </h4>
                            </div>
                        </div>
                    </div>
                </DetailContainer>
                <DetailContainer title="Experience".to_owned()>
                    <div class="sub-card-container">
                        <ExperienceDetails />
                    </div>
                </DetailContainer>

                <DetailContainer title="Skills".to_owned()>
                    <div class="sub-card-container" style="margin: 0px; padding: 5px">
                        <SkillsDetails />
                    </div>
                </DetailContainer>

                <DetailContainer title="Projects".to_owned()>
                    <div class="sub-card-container" style="margin: 0px; padding: 5px">
                        <ProjectsDetails />
                    </div>
                </DetailContainer>

                <DetailContainer title="Contact Me".to_owned() open=true>
                    <div class="sub-card-container" style="margin: 0px; padding: 5px">
                        <ContactDetails />
                    </div>
                </DetailContainer>
            </div>
        </div>
    }.into_any()
}

#[component]
fn TestHomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <div class="parallax">
                <div class="blurred-text">
                    <h1 class="extra-large">
                        "Hi, I'm "<span class="custom-text-accent">"Chris Bratti."</span>
                    </h1>
                    <h4 class="subtitle">"${ s o f t w a r e _ e n g i n e e r }"</h4>
                </div>
                <div class="down-arrow">"⇓"</div>
            </div>
            <div class="blurred-backdrop">
                <AboutContainer />
            </div>
        </div>
    }
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
fn ExperienceDetails() -> impl IntoView {
    let pdf_link = Resource::new_blocking(|| (), |_| generate_pdf_link());
    view! {
        <div class="experience-card">
            <div class="experience-card-title">"Booz Allen Hamilton"</div>
            <h4>
                <span class="text-italic">
                    "Software Engineer, Senior Consultant [Aug 2021 - Present]"
                </span>
            </h4>
            <ul class="modern-list">
                <li>"Develop and maintain suite of Spring Boot RESTful APIs"</li>
                <li>"Lead functionality implementation and project architecture planning"</li>
                <li>
                    "Collaborate with Scrum team to deliver fully-tested, production ready solutions"
                </li>
                <li>
                    "Apply Test-Driven Development principles using JUnit, maintaining 95%+ code coverage"
                </li>
                <li>"Deploy applications into Highly-Available EKS Kubernetes environment"</li>
                <li>
                    "Automate workflows and processes by building tooling with Bash, Rust, and Python"
                </li>
                <li>"Mentor junior engineers on product expertise and coding standards"</li>
            </ul>
        </div>
        <div class="experience-card">
            <div class="experience-card-title">"The Hartford"</div>
            <h4>
                <span class="text-italic">"Associate Software Engineer [Jan 2019 - Aug 2021]"</span>
            </h4>
            <ul class="modern-list">
                <li>"Supported the development of PolicyCenter, a core insurance platform"</li>
                <li>
                    "Developed Python scripts to automate error detection and improve workflows"
                </li>
                <li>"Spearheaded migration of legacy SOAP services to modern RESTful APIs"</li>
                <li>"Integrated PolicyCenter with other applications within the Hartford"</li>
                <li>"Automated API testing using the Karate framework"</li>
                <li>"Designed and optimized reusable SQL queries for ad hoc business needs"</li>
            </ul>
        </div>
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
fn SkillComponent(title: &'static str, skills: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="experience-card">
            <div class="experience-card-title">{title}</div>
            <ul class="modern-list">
                {skills.into_iter()
                    .map(|skill| view! { <li>{skill}</li>})
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn SkillsDetails() -> impl IntoView {
    let langues = vec!["Java", "Rust", "Bash", "Python", "SQL"];
    let frameworks = vec![
        "Spring Boot",
        "jUnit",
        "AssertJ",
        "Mockito",
        "Leptos",
        "Actix Web",
        "Helm",
        "OAuth",
    ];
    let devops = vec![
        "Kubernetes",
        "Docker",
        "AWS",
        "Jenkins",
        "ArgoCD",
        "Hashicorp",
    ];
    let dev_tools = vec![
        "Git",
        "GitHub",
        "Swagger / OpenAPI",
        "Postman",
        "Bruno",
        "IntelliJ",
    ];
    view! {
        <SkillComponent title="Languages" skills=langues/>
        <SkillComponent title="Tools and Frameworks" skills=frameworks/>
        <SkillComponent title="DevOps and Deployment" skills=devops/>
        <SkillComponent title="Development Tools" skills=dev_tools/>
    }
}

#[component]
fn GitHubLink(project_name: String, text: Option<String>) -> impl IntoView {
    view! {
        <a
                class="btn github-link"
                target="_blank"
                rel="noopener noreferrer"
                href={format!("https://github.com/chris-bratti/{}", project_name)}
            >
            {
                if text.is_none(){
                    "View on GitHub!".to_string()
                }else{
                    text.unwrap().to_string()
                }
            } <img src="/assets/github-mark.png" class="github-icon"/>
            </a>
    }
}

#[component]
fn ProjectsDetails() -> impl IntoView {
    view! {
        <div class="experience-card">
            <div class="experience-card-title">"Auth-server"</div>
            <span class="text-italic">
                    "A full stack Rust OAuth server written with the Leptos framework"
            </span>
            <h5>
                "
                This project initially began as "
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/chris-bratti/auth_leptos"
                >
                    "auth_leptos"
                </a>", a full stack user authentication application.
                What started as a desire to get more hands-on experience with authentication systems has turned into a full OAuth server. The included docker-compose.yaml file makes it
                easy to deploy into a Docker or Kubernetes environment. Integrating new OAuth clients is simple and secure - clients can sign up using the REST endpoint and admins can
                approve or deny new clients in the Admin Dashboard.
                "
            </h5>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Features"</span>
            </h4>
            <ul class="modern-list" style="font-size: .85em">
                <li>
                    "Full OAuth Authorization Code flow - easy to integrate and protect applications"
                </li>
                <li>"Secure user signup, login, and password reset"</li>
                <li>"Persistent user session storage with actix_sessions"</li>
                <li>"Two factor authentication with Time-Based One Time Passwords (TOTP)"</li>
                <li>"Admin Dashboard"</li>
                <li>"Database hashing and encryption"</li>
                <li>"Redis integration for application caching"</li>
            </ul>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Technologies/skills used"</span>
            </h4>
            <ul class="modern-list" style="font-size: .85em">
                <li>"Rust, Leptos, Actix Web"</li>
                <li>"WASM, HTML, CSS"</li>
                <li>"Diesel, Postgres, Redis"</li>
                <li>"Argon2 hashing, database encryption"</li>
                <li>"Docker"</li>
            </ul>
            <GitHubLink project_name="auth-server".to_string() text=None/>
        </div>
        <div class="experience-card">
            <div class="experience-card-title">"Resume Website"</div>
            <span class="text-italic">
                    "Full stack resume and portfolio site (you're using it right now!)"
            </span>
            <h5>
                "
                The repository for this site: a full-stack portfolio and resume website written in Rust. With a WASM-compiled front-end and an Actix-Web server, this site is 'blazingly fast'
                with minimal overhead.
                "
            </h5>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Features"</span>
            </h4>
            <ul class="modern-list" style="font-size: .85em">
                <li>"Eye-catching UI designed with Leptos"</li>
                <li>"Fast and lightweight back-end written in Rust with Leptos and Actix"</li>
                <li>"Secure SMTP communication with TLS"</li>
                <li>"Protected download links"</li>
            </ul>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Technologies/skills used"</span>
            </h4>
            <ul class="modern-list" style="font-size: .85em">
                <li>"Rust, Leptos, Actix Web"</li>
                <li>"WASM, HTML, CSS"</li>
                <li>"Docker"</li>
            </ul>
            <GitHubLink project_name="chrisbratti-website".to_string() text=None/>
        </div>
        <div class="experience-card">
            <div class="experience-card-title">"Wireguard-init"</div>
            <span class="text-italic">
                    "Script to automate the creation of a WireGuard server and peers"
            </span>
            <h5>
                "A lightweight and easy-to-use script to automate initializing, configuring, and deploying a WireGuard server and peers. I've developed and used this script to deploy several WireGuard servers to
                enable secure remote communication with my personal network when I'm away from home."
            </h5>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Features"</span>
            </h4>
            <ul class="modern-list" style="font-size: .85em">
                <li>"Creates a secure peer-to-peer VPN network in just a few minutes"</li>
                <li>
                    "Ability to customize IP address subnet, listening port, DNS server addresses, and more"
                </li>
                <li>"Automates peer/client setup"</li>
                <li>"Generates QR codes for configuring mobile peers"</li>
                <li>"Lightweight"</li>
            </ul>
            <h4 style="margin-top: 15px">
                <span class="custom-text-accent">"Technologies/skills used"</span>
            </h4>
            <ul class="modern-list" style="font-size: .85em">
                <li>"Bash"</li>
                <li>"Networking"</li>
                <li>"Routing"</li>
            </ul>
            <GitHubLink project_name="wireguard-init".to_string() text=None/>
        </div>
        <div class="experience-card" style="text-align: center">
            <GitHubLink project_name="".to_string() text=Some("Check out the rest of my projects on GitHub!".to_string())/>
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
                            <span class="custom-text-accent"><i class="material-icons in-line-icon">mail</i></span>
                            {format!("{}", info.email)}
                        </p>
                        <p>
                            <span class="custom-text-accent"><i class="material-icons in-line-icon">account_circle</i></span>
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

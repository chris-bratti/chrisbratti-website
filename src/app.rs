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
        <Title text="Welcome to ChrisBratti.com" />

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
fn Overview() -> impl IntoView {
    view! {
        <div class="overview-container">
            <div class="overview-card" style="background-image: url('/assets/house-left.jpg');">
                <div class="blurred-text">
                    <h1>"8 Years"</h1>
                    <h2>"Developing With Java"</h2>
                </div>
            </div>
            <div class="overview-card" style="background-image: url('/assets/house-center.jpg');">
                <div class="blurred-text">
                    <h1>"6 Years"</h1>
                    <h2>"Professional Experience"</h2>
                </div>
            </div>
            <div class="overview-card" style="background-image: url('/assets/house-right.jpg');">
                <div class="blurred-text">
                    <h1>"5 Years"</h1>
                    <h2>"API Development"</h2>
                </div>
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn DetailContainer(title: String, children: Children, #[prop(optional)] open: bool) -> impl IntoView {
    view! {
        <details class="expandable-card" open=open>
            <summary>{title}</summary>
            <div class="card-content">
                {children()}
            </div>
        </details>
        
    }
}

#[component]
fn AboutContainer() -> impl IntoView {
    let info_result = Resource::new_blocking(|| (), |_| get_info());
    let pdf_link = Resource::new_blocking(|| (), |_| generate_pdf_link());
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
                        <div class="experience-card">
                            <h3>
                                <span class="custom-text-accent">"Booz Allen Hamilton"</span>
                            </h3>
                            <h4>
                                <span class="text-italic">
                                    "Software Engineer, Senior Consultant [Aug 2021 - Present]"
                                </span>
                            </h4>
                            <ul class="icon-list">
                                <li>"Develop and maintain suite of Spring Boot RESTful APIs"</li>
                                <li>
                                    "Lead functionality implementation and project architecture planning"
                                </li>
                                <li>
                                    "Collaborate with Scrum team to deliver fully-tested, production ready solutions"
                                </li>
                                <li>
                                    "Apply Test-Driven Development principles using JUnit, maintaining 95%+ code coverage"
                                </li>
                                <li>
                                    "Deploy applications into Highly-Available EKS Kubernetes environment"
                                </li>
                                <li>
                                    "Automate workflows and processes by building tooling with Bash, Rust, and Python"
                                </li>
                                <li>
                                    "Mentor junior engineers on product expertise and coding standards"
                                </li>
                            </ul>
                        </div>
                        <div class="experience-card">
                            <h3>
                                <span class="custom-text-accent">"The Hartford"</span>
                            </h3>
                            <h4>
                                <span class="text-italic">
                                    "Associate Software Engineer [Jan 2019 - Aug 2021]"
                                </span>
                            </h4>
                            <ul class="icon-list">
                                <li>
                                    "Supported the development of PolicyCenter, a core insurance platform"
                                </li>
                                <li>
                                    "Developed Python scripts to automate error detection and improve workflows"
                                </li>
                                <li>
                                    "Spearheaded migration of legacy SOAP services to modern RESTful APIs"
                                </li>
                                <li>
                                    "Integrated PolicyCenter with other applications within the Hartford"
                                </li>
                                <li>"Automated API testing using the Karate framework"</li>
                                <li>
                                    "Designed and optimized reusable SQL queries for ad hoc business needs"
                                </li>
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
                    </div>
                </DetailContainer>

                <DetailContainer title="Skills".to_owned()>
                    <div class="sub-card-container" style="margin: 0px; padding: 5px">
                        <div class="experience-card">
                            <ul class="icon-list">
                                <li>"Java, Rust, Bash, Python, SQL"</li>
                                <li>"Spring Boot, jUnit, AssertJ, Mockito"</li>
                                <li>"Docker, Kubernetes, AWS, Jenkins, ArgoCD"</li>
                                <li>"Git, GitHub, Swagger, Postman, IntelliJ"</li>
                            </ul>
                        </div>
                    </div>
                </DetailContainer>

                <DetailContainer title="Projects".to_owned()>
                    <div class="sub-card-container" style="margin: 0px; padding: 5px">
                        <div class="experience-card">
                            <h2>"auth-server"</h2>
                            <h4>"A full stack Rust OAuth server written with the Leptos framework"</h4>
                            <h5>"
                            This project initially began as "<a target="_blank" rel="noopener noreferrer" href="https://github.com/chris-bratti/auth_leptos">"auth_leptos"</a>", a full stack user authentication application.
                            What started as a desire to get more hands-on experience with authentication systems has turned into a full OAuth server. The included docker-compose.yaml file makes it
                            easy to deploy into a Docker or Kubernetes environment. Integrating new OAuth clients is simple and secure - clients can sign up using the REST endpoint and admins can
                            approve or deny new clients in the Admin Dashboard.
                            "</h5>
                            <h5>"Features:"</h5>
                            <ul class="modern-list" style="font-size: .85em">
                                <li>"Full OAuth Authorization Code flow - easy to integrate and protect applications"</li>
                                <li>"Secure user signup, login, and password reset"</li>
                                <li>"Persistent user session storage with actix_sessions"</li>
                                <li>"Two factor authentication with Time-Based One Time Passwords (TOTP)"</li>
                                <li>"Admin Dashboard"</li>
                                <li>"Database hashing and encryption"</li>
                                <li>"Redis integration for application caching"</li>
                            </ul>
                            <h5>"Technologies/skills used:"</h5>
                            <ul class="modern-list" style="font-size: .85em">
                                <li>"Rust, Leptos, Actix Web"</li>
                                <li>"WASM, HTML, CSS"</li>
                                <li>"Diesel, Postgres, Redis"</li>
                                <li>"Argon2 hashing, database encryption"</li>
                                <li>"Docker"</li>
                            </ul>
                            <a class="btn" target="_blank" rel="noopener noreferrer" href="https://github.com/chris-bratti/auth-server">"View on GitHub!"</a>
                        </div>
                        <div class="experience-card">
                            <h2>"chrisbratti-website"</h2>
                            <h4>"Full stack resume and portfolio site (you're using it right now!)"</h4>
                            <h5>"
                            The repository for this site: a full-stack portfolio and resume website written in Rust. With a WASM-compiled front-end and an Actix-Web server, this site is 'blazingly fast'
                            with minimal overhead.
                            "</h5>
                            <h5>"Features:"</h5>
                            <ul class="modern-list" style="font-size: .85em">
                                <li>"Eye-catching UI designed with Leptos"</li>
                                <li>"Fast and lightweight back-end written in Rust with Leptos and Actix"</li>
                                <li>"Secure SMTP communication with TLS"</li>
                                <li>"Protected download links"</li>
                            </ul>
                            <h5>"Technologies/skills used:"</h5>
                            <ul class="modern-list" style="font-size: .85em">
                                <li>"Rust, Leptos, Actix Web"</li>
                                <li>"WASM, HTML, CSS"</li>
                                <li>"Docker"</li>
                            </ul>
                            <a class="btn" target="_blank" rel="noopener noreferrer" href="https://github.com/chris-bratti/chrisbratti-website">"View on GitHub!"</a>
                        </div>
                        <div class="experience-card">
                            <h2>"wireguard-init"</h2>
                            <h4>"Script to automate the creation of a WireGuard server and peers"</h4>
                            <h5>"A lightweight and easy-to-use script to automate initializing, configuring, and deploying a WireGuard server and peers. I've developed and used this script to deploy several WireGuard servers to
                            enable secure remote communication with my personal network when I'm away from home."</h5>
                            <h5>"Features:"</h5>
                            <ul class="modern-list" style="font-size: .85em">
                                <li>"Creates a secure peer-to-peer VPN network in just a few minutes"</li>
                                <li>"Ability to customize IP address subnet, listening port, DNS server addresses, and more"</li>
                                <li>"Automates peer/client setup"</li>
                                <li>"Generates QR codes for configuring mobile peers"</li>
                                <li>"Lightweight"</li>
                            </ul>
                            <h5>"Technologies/skills used:"</h5>
                            <ul class="modern-list" style="font-size: .85em">
                                <li>"Bash"</li>
                                <li>"Networking"</li>
                                <li>"Routing"</li>
                            </ul>
                            <a target="_blank" rel="noopener noreferrer" href="https://github.com/chris-bratti/wireguard-init">"View on GitHub!"</a>
                        </div>
                        <div class="experience-card" style="text-align: center">
                            <a class="btn" target="_blank" rel="noopener noreferrer" href="https://github.com/chris-bratti" style="font-size:1em">"Check out the rest of my projects on GitHub!"</a>
                        </div>
                    </div>
                </DetailContainer>

                <DetailContainer title="Contact Me".to_owned() open=true>
                    <div class="sub-card-container" style="margin: 0px; padding: 5px">
                        <div class="experience-card">
                            <ContactForm />
                        </div>
                        <div class="experience-card">
                            <h3 style="margin-bottom: 5px;font-family: 'Abril Fatface', serif;color: #bfc8a4;">
                                "Or reach me here!"
                            </h3>
                            <Suspense fallback=|| ()>
                                {move || Suspend::new(async move {
                                    let info = info_result.await.unwrap();
                                    view! {
                                        <p>
                                            <i class="material-icons in-line-icon">mail</i>
                                            {format!("{}", info.email)}
                                        </p>
                                        <p>
                                            <i class="material-icons in-line-icon">account_circle</i>
                                            {format!("{}", info.linkedin)}
                                        </p>
                                    }
                                })}
                            </Suspense>
                        </div>
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
                <h1 class="extra-large">
                    "Hi, I'm "<span class="custom-text-accent">"Chris Bratti."</span>
                </h1>
                <h4 class="subtitle">"${ s o f t w a r e _ e n g i n e e r }"</h4>
                <div class="down-arrow">"⇓"</div>
            </div>
            <div class="blurred-backdrop">
                //<Overview />
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
                    <h3 style="margin-bottom: 5px;font-family: 'Abril Fatface', serif;color: #bfc8a4;">
                        "Send me an email!"
                    </h3>
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
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

use leptos::{prelude::ServerFnError, server};

#[server(SendEmail, "/api")]
pub async fn send_email(
    first_name: String,
    last_name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    println!("{first_name} {last_name} {email} {message}");
    Ok(())
}

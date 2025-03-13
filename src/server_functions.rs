use leptos::{prelude::ServerFnError, server};

#[server(SendEmail, "/api")]
pub async fn send_email() -> Result<(), ServerFnError> {
    Ok(())
}

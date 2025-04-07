use crate::services::resume_parsing_service::parse_resume;
use crate::services::resume_parsing_service::save_resume_json;
use crate::services::resume_parsing_service::update_current_resume;
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures_util::StreamExt;

// Upload resume
#[cfg(feature = "ssr")]
#[actix_web::post("/internal/resume/update")]
pub async fn upload_resume(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(field) = payload.next().await {
        let mut field = field?;

        // Content type has to be PDF
        let content_type = field
            .content_type()
            .ok_or_else(|| {
                actix_web::error::ErrorBadRequest("Content header could not be determined")
            })?
            .to_string();
        if content_type != "application/pdf" {
            return Err(actix_web::error::ErrorBadRequest("File must be PDF"));
        }

        let mut file_bytes = Vec::new();

        while let Some(chunk) = field.next().await {
            file_bytes.extend_from_slice(&chunk.map_err(|_| {
                actix_web::error::ErrorInternalServerError("Error reading file chunk")
            })?);
        }

        // Save PDF in background
        let update_handle = tokio::spawn({
            let file_bytes = file_bytes.clone();
            async move { update_current_resume(file_bytes).await }
        });

        // Call resume parsing service
        let response = parse_resume(file_bytes.clone())
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error parsing resume!"))?;

        // Save the response as JSON
        save_resume_json(&response)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error saving resume JSON"))?;

        // Wait for PDF task to finish
        update_handle
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("PDF save task panicked"))?
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error saving PDF file"))?;

        return Ok(HttpResponse::Ok().json(response));
    }

    // If here then payload was likely bad
    Err(actix_web::error::ErrorBadRequest("Error uploading resume"))
}

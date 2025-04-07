/*

func upload_resume(){
    // Upload resume
    // Save resume in file structure (Increment filenames)
    // Hit parseCV, get JSON response
    // Save JSON response in database? File?

    // On app start, load JSON from DB/file and add as AppData in Actix
}


*/
use crate::services::resume_parsing_service::parse_resume;
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures_util::StreamExt;

// Upload resume
#[cfg(feature = "ssr")]
#[actix_web::post("/internal/resume/update")]
pub async fn upload_resume(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    use crate::services::resume_parsing_service::update_current_resume;

    while let Some(field) = payload.next().await {
        let mut field = field?;
        let content_type = field
            .content_type()
            .ok_or_else(|| {
                actix_web::error::ErrorBadRequest("Content header could not be determined")
            })?
            .to_string();
        if content_type != "application/pdf" {
            return Err(actix_web::error::ErrorBadRequest("File must be PDF"));
        }

        let mut file_bytes = Vec::<u8>::new();

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_bytes.append(&mut data.to_vec());
        }
        update_current_resume(&file_bytes).await?;
        let response = parse_resume(file_bytes).await?;
        return Ok(HttpResponse::Ok().json(response));
    }
    // TODO - Load resume into memory

    Err(actix_web::error::ErrorInternalServerError(
        "Error uploading resume",
    ))
}

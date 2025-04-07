use std::{io::Write, path::PathBuf};

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::Client;
use std::fs;

use crate::{server_functions::get_env_variable, Resume};

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::new();
    static ref PARSE_URL: String = get_env_variable("PARSE_URL").expect("PARSE_URL not set!");
    static ref PARSE_API_KEY: String =
        get_env_variable("PARSE_API_KEY").expect("PARSE_API_KEY not set!");
    static ref RESUME_FILE_NAME: String =
        get_env_variable("RESUME_FILE_NAME").expect("RESUME_FILE_NAME not set!");
}

pub async fn parse_resume(file_bytes: Vec<u8>) -> Result<Resume, Box<dyn std::error::Error>> {
    let form = reqwest::multipart::Form::new().part(
        "resume",
        reqwest::multipart::Part::bytes(file_bytes)
            .file_name("resume.pdf")
            .mime_str("application/pdf")?,
    );

    let res = CLIENT
        .post(format!("{}/api/parse", PARSE_URL.to_string()))
        .header("apiKey", PARSE_API_KEY.to_string())
        .multipart(form)
        .send()
        .await?;

    if res.status() != reqwest::StatusCode::OK {
        println!("parseCV error: {}", res.text().await.unwrap());
        return Err("Bad response from parseCV service!".into());
    }

    let resume = res.json::<Resume>().await?;

    Ok(resume)
}

pub async fn update_current_resume(
    new_resume_bytes: &Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let curr_resume_path_str = format!("uploads/{}.pdf", RESUME_FILE_NAME.to_string());
    let curr_resume_path: PathBuf = curr_resume_path_str.clone().into();

    if curr_resume_path.is_file() {
        let creation_date: DateTime<Utc> = curr_resume_path.metadata()?.created()?.into();
        let updated_file_name = format!(
            "{}_{}.pdf",
            curr_resume_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".pdf", ""),
            creation_date.format("%d-%m-%Y")
        );
        println!("Copying file");
        let backup_name = sanitize_filename::sanitize(updated_file_name);
        let backup_path = PathBuf::from(format!("uploads/{}", backup_name));
        fs::copy(&curr_resume_path, &backup_path)?;
    }

    println!("Overwriting resume file {}", curr_resume_path_str);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&curr_resume_path_str)?;

    file.write_all(new_resume_bytes)?;

    Ok(())
}

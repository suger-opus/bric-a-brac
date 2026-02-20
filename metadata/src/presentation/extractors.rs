use super::errors::RequestError;
use crate::domain::models::UserId;
use axum::{
    body::Body,
    extract::{FromRequest, FromRequestParts, Request},
    http::request::Parts,
};
use futures_util::TryStreamExt;
use utoipa::ToSchema;

const MAX_FILE_SIZE: usize = 100 * 1024; // 100KB

pub struct AuthenticatedUser {
    pub user_id: UserId,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = RequestError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("Extracting authenticated user from request");

        let user_id_str = parts
            .headers
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| RequestError::Unauthorized {
                reason: "Missing user_id header".to_string(),
            })?;

        let user_id = user_id_str
            .parse::<UserId>()
            .map_err(|_| RequestError::Unauthorized {
                reason: "Invalid user_id format - must be a valid UUID".to_string(),
            })?;

        tracing::debug!(user_id = ?user_id, "Successfully extracted authenticated user");
        Ok(AuthenticatedUser { user_id })
    }
}

#[derive(Debug, ToSchema)]
pub struct MultipartFileUpload {
    pub file_content: Vec<u8>,
    pub file_type: String,
}

impl<S> FromRequest<S> for MultipartFileUpload
where
    S: Send + Sync,
{
    type Rejection = RequestError;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("Extracting multipart file upload from request");

        let headers = req.headers();
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| RequestError::InvalidHeader {
                issue: "Missing content-type header".to_string(),
                header: "content-type".to_string(),
            })?;

        let boundary =
            multer::parse_boundary(content_type).map_err(|_| RequestError::InvalidHeader {
                issue: "Invalid content-type".to_string(),
                header: "content-type".to_string(),
            })?;

        let stream = req
            .into_body()
            .into_data_stream()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        let mut multipart = multer::Multipart::new(stream, boundary);

        let mut file_content: Option<Vec<u8>> = None;
        let mut file_type: Option<String> = None;

        while let Some(field) =
            multipart
                .next_field()
                .await
                .map_err(|err| RequestError::InvalidFile {
                    issue: format!("Failed to read field: {}", err),
                })?
        {
            let name = field.name().unwrap_or("").to_string();

            match name.as_str() {
                "file" => {
                    let data = field
                        .bytes()
                        .await
                        .map_err(|err| RequestError::InvalidFile {
                            issue: format!("Failed to read file: {}", err),
                        })?;

                    if data.len() > MAX_FILE_SIZE {
                        return Err(RequestError::InvalidFile {
                            issue: format!(
                                "File size exceeds maximum of {}KB",
                                MAX_FILE_SIZE / 1024
                            ),
                        });
                    }

                    file_content = Some(data.to_vec());
                }
                "file_type" => {
                    let value = field
                        .text()
                        .await
                        .map_err(|err| RequestError::InvalidFile {
                            issue: format!("Failed to read file_type: {}", err),
                        })?
                        .to_lowercase();

                    if value != "txt" && value != "csv" {
                        return Err(RequestError::InvalidFile {
                            issue: format!("Invalid file_type {value}"),
                        });
                    }

                    file_type = Some(value);
                }
                _ => {}
            }
        }

        let file_content = file_content.ok_or_else(|| RequestError::InvalidFile {
            issue: "Missing 'file' field".to_string(),
        })?;
        let file_type = file_type.ok_or_else(|| RequestError::InvalidFile {
            issue: "Missing 'file_type' field".to_string(),
        })?;

        tracing::debug!(
            file_type = %file_type,
            "Successfully extracted multipart file upload",
        );
        Ok(MultipartFileUpload {
            file_content,
            file_type,
        })
    }
}

use super::error::{AppError, DomainError};
use crate::domain::models::UserId;
use axum::{
    body::Body,
    extract::{FromRequest, FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use futures_util::TryStreamExt;

const MAX_FILE_SIZE: usize = 100 * 1024; // 100KB

pub struct AuthenticatedUser {
    pub user_id: UserId,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id_str = parts
            .headers
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                AppError::Domain(DomainError::Unauthorized {
                    reason: "Missing user_id header".to_string(),
                })
            })?;

        let user_id = user_id_str.parse::<UserId>().map_err(|_| {
            AppError::Domain(DomainError::Unauthorized {
                reason: "Invalid user_id format - must be a valid UUID".to_string(),
            })
        })?;

        Ok(AuthenticatedUser { user_id })
    }
}

pub struct MultipartFileUpload(pub Vec<u8>, pub String);

impl<S> FromRequest<S> for MultipartFileUpload
where
    S: Send + Sync,
{
    type Rejection = axum::http::Response<axum::body::Body>;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, "Missing content-type header").into_response()
            })?;

        let boundary = multer::parse_boundary(content_type)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid content-type").into_response())?;

        let stream = req
            .into_body()
            .into_data_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        let mut multipart = multer::Multipart::new(stream, boundary);

        let mut file_content: Option<Vec<u8>> = None;
        let mut file_type: Option<String> = None;

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to read field: {}", e),
            )
                .into_response()
        })? {
            let name = field.name().unwrap_or("").to_string();

            match name.as_str() {
                "file" => {
                    let data = field.bytes().await.map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            format!("Failed to read file: {}", e),
                        )
                            .into_response()
                    })?;

                    if data.len() > MAX_FILE_SIZE {
                        return Err((
                            StatusCode::PAYLOAD_TOO_LARGE,
                            format!("File size exceeds maximum of {}KB", MAX_FILE_SIZE / 1024),
                        )
                            .into_response());
                    }

                    file_content = Some(data.to_vec());
                }
                "file_type" => {
                    let value = field.text().await.map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            format!("Failed to read file_type: {}", e),
                        )
                            .into_response()
                    })?;

                    file_type = Some(value.to_lowercase());
                }
                _ => {}
            }
        }

        let file_content = file_content
            .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing 'file' field").into_response())?;
        let file_type = file_type.ok_or_else(|| {
            (StatusCode::BAD_REQUEST, "Missing 'file_type' field").into_response()
        })?;

        Ok(MultipartFileUpload(file_content, file_type))
    }
}

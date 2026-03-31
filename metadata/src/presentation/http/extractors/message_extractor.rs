use crate::presentation::error::PresentationError;
use axum::extract::{FromRequest, Multipart, Request};
use bric_a_brac_dtos::SessionIdDto;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB

pub struct ChatMessageUpload {
    pub session_id: SessionIdDto,
    /// User text content, `None` if omitted or blank.
    pub content: Option<String>,
    /// Extracted plain text from the uploaded file, if any.
    pub file_text: Option<String>,
    pub file_name: Option<String>,
}

impl<S> FromRequest<S> for ChatMessageUpload
where
    S: Send + Sync,
{
    type Rejection = PresentationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await.map_err(|err| {
            PresentationError::InvalidMultipartFile {
                reason: err.to_string(),
            }
        })?;

        let mut session_id: Option<String> = None;
        let mut content: Option<String> = None;
        let mut file_text: Option<String> = None;
        let mut file_name: Option<String> = None;

        while let Some(mut field) = multipart
            .next_field()
            .await
            .map_err(|err| PresentationError::MultipartReadError { source: err })?
        {
            let name = field.name().unwrap_or_default().to_owned();
            match name.as_str() {
                "session_id" => {
                    session_id =
                        Some(field.text().await.map_err(|err| {
                            PresentationError::MultipartReadError { source: err }
                        })?);
                }
                "content" => {
                    content =
                        Some(field.text().await.map_err(|err| {
                            PresentationError::MultipartReadError { source: err }
                        })?);
                }
                "file" => {
                    let name = field.file_name().map(ToString::to_string);
                    let content_type = field.content_type().map(ToString::to_string);
                    let bytes = read_limited(&mut field).await?;
                    let extracted = extract_text(&bytes, content_type.as_deref(), name.as_deref())?;
                    file_name = name;
                    file_text = Some(extracted);
                }
                _ => {}
            }
        }

        let session_id = session_id
            .ok_or_else(|| PresentationError::MissingMultipartField {
                field: "session_id".to_owned(),
            })?
            .try_into()?;

        let content = content.filter(|s| !s.is_empty());
        if content.is_none() && file_text.is_none() {
            return Err(PresentationError::MissingMultipartField {
                field: "content or file".to_owned(),
            });
        }

        Ok(Self {
            session_id,
            content,
            file_text,
            file_name,
        })
    }
}

enum FileType {
    Pdf,
    PlainText,
}

impl FileType {
    fn from_content_type(content_type: Option<&str>, filename: Option<&str>) -> Option<Self> {
        if let Some(ct) = content_type {
            match ct {
                "application/pdf" => return Some(Self::Pdf),
                "text/plain" => return Some(Self::PlainText),
                _ => {}
            }
        }

        let extension = std::path::Path::new(filename?).extension();
        if extension.is_some_and(|ext| ext.eq_ignore_ascii_case("pdf")) {
            Some(Self::Pdf)
        } else if extension.is_some_and(|ext| ext.eq_ignore_ascii_case("txt")) {
            Some(Self::PlainText)
        } else {
            None
        }
    }
}

/// Reads a multipart field in chunks, returning an error as soon as the
/// accumulated size exceeds `MAX_FILE_SIZE`. This avoids loading huge uploads
/// into memory before the limit is checked.
async fn read_limited(
    field: &mut axum::extract::multipart::Field<'_>,
) -> Result<Vec<u8>, PresentationError> {
    let mut buf = Vec::new();
    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|err| PresentationError::MultipartReadError { source: err })?
    {
        if buf.len() + chunk.len() > MAX_FILE_SIZE {
            return Err(PresentationError::InvalidMultipartFile {
                #[allow(clippy::cast_precision_loss)]
                reason: format!(
                    "File too large (> {:.1} MB). Maximum is {} MB",
                    MAX_FILE_SIZE as f64 / 1_048_576.0,
                    MAX_FILE_SIZE / 1_048_576
                ),
            });
        }
        buf.extend_from_slice(&chunk);
    }

    Ok(buf)
}

fn extract_text(
    bytes: &[u8],
    content_type: Option<&str>,
    filename: Option<&str>,
) -> Result<String, PresentationError> {
    if bytes.is_empty() {
        return Err(PresentationError::InvalidMultipartFile {
            reason: "File is empty".to_owned(),
        });
    }

    let file_type = FileType::from_content_type(content_type, filename).ok_or(
        PresentationError::InvalidMultipartFile {
            reason: format!(
                "Unsupported file type. Got content_type={}, filename={}. Supported: PDF (.pdf), plain text (.txt)",
                content_type.unwrap_or("none"),
                filename.unwrap_or("none"),
            ),
        },
    )?;

    match file_type {
        FileType::Pdf => {
            let text = pdf_extract::extract_text_from_mem(bytes).map_err(|err| {
                PresentationError::MultipartFileReadError {
                    source: Some(Box::new(err)),
                }
            })?;
            Ok(text.replace('\0', ""))
        }
        FileType::PlainText =>
        {
            #[allow(clippy::map_err_ignore)]
            String::from_utf8(bytes.to_vec()).map_err(|_| PresentationError::InvalidMultipartFile {
                reason: "File is not valid UTF-8 text".to_owned(),
            })
        }
    }
}

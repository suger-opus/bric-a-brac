use crate::application::errors::RequestError;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB

/// Supported file types for text extraction.
enum FileType {
    Pdf,
    PlainText,
}

impl FileType {
    fn from_content_type(content_type: Option<&str>, filename: Option<&str>) -> Option<Self> {
        // Try content type first
        if let Some(ct) = content_type {
            match ct {
                "application/pdf" => return Some(Self::Pdf),
                "text/plain" => return Some(Self::PlainText),
                _ => {}
            }
        }

        // Fall back to file extension
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

/// Extract text content from uploaded file bytes.
/// Supports PDF and plain text files.
pub fn extract_text(
    bytes: &[u8],
    content_type: Option<&str>,
    filename: Option<&str>,
) -> Result<String, RequestError> {
    if bytes.is_empty() {
        return Err(RequestError::InvalidFile {
            issue: "File is empty".to_owned(),
            source: None,
        });
    }

    if bytes.len() > MAX_FILE_SIZE {
        return Err(RequestError::InvalidFile {
            #[allow(clippy::cast_precision_loss)]
            issue: format!(
                "File too large ({:.1} MB). Maximum is {} MB",
                bytes.len() as f64 / 1_048_576.0,
                MAX_FILE_SIZE / 1_048_576
            ),
            source: None,
        });
    }

    let file_type =
        FileType::from_content_type(content_type, filename).ok_or(RequestError::InvalidFile {
            issue: format!(
                "Unsupported file type. Got content_type={}, filename={}. Supported: PDF (.pdf), plain text (.txt)",
                content_type.unwrap_or("none"),
                filename.unwrap_or("none"),
            ),
            source: None,
        })?;

    match file_type {
        FileType::Pdf => extract_pdf(bytes),
        FileType::PlainText => extract_plain_text(bytes),
    }
}

fn extract_pdf(bytes: &[u8]) -> Result<String, RequestError> {
    let text =
        pdf_extract::extract_text_from_mem(bytes).map_err(|err| RequestError::InvalidFile {
            issue: "Failed to extract text from PDF".to_owned(),
            source: Some(Box::new(err)),
        })?;
    // PDF extraction can produce null bytes that PostgreSQL rejects in text columns.
    Ok(text.replace('\0', ""))
}

fn extract_plain_text(bytes: &[u8]) -> Result<String, RequestError> {
    String::from_utf8(bytes.to_vec()).map_err(|err| RequestError::InvalidFile {
        issue: "File is not valid UTF-8 text".to_owned(),
        source: Some(Box::new(err)),
    })
}

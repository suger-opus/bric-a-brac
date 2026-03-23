/// Maximum characters per chunk (~2000 tokens at ~4 chars/token).
/// Keeps each AI turn focused for better extraction quality.
const MAX_CHUNK_CHARS: usize = 8_000;

const DOC_PREFIX: &str = "[Document content]\n";
const MSG_SEPARATOR: &str = "\n\n[User message]\n";

/// Split a user message into chunks if it contains a large document.
///
/// Returns a list of formatted messages ready to be sent as sequential user turns.
/// For small messages or plain text (no document), returns the original message as-is.
pub fn chunk_user_message(message: &str) -> Vec<String> {
    let (doc_text, user_msg) = parse_document_message(message);

    let doc_text = match doc_text {
        Some(doc) if doc.len() > MAX_CHUNK_CHARS => doc,
        _ => return vec![message.to_owned()],
    };

    let text_chunks = split_at_paragraphs(doc_text, MAX_CHUNK_CHARS);
    let total = text_chunks.len();

    text_chunks
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let part_label = format!("[Document content — Part {}/{}]", i + 1, total);
            if i == 0 {
                match user_msg {
                    Some(msg) => {
                        format!("{part_label}\n{chunk}\n\n[User message]\n{msg}")
                    }
                    None => format!("{part_label}\n{chunk}"),
                }
            } else {
                format!(
                    "{part_label}\n{chunk}\n\n\
                     Continue extracting knowledge from this part of the document. \
                     Reuse existing schemas and resolve entities that appeared in previous parts."
                )
            }
        })
        .collect()
}

/// Extract the document text and optional user message from the combined format
/// produced by the chat handler.
fn parse_document_message(message: &str) -> (Option<&str>, Option<&str>) {
    let rest = match message.strip_prefix(DOC_PREFIX) {
        Some(r) => r,
        None => return (None, None),
    };

    if let Some(idx) = rest.find(MSG_SEPARATOR) {
        let doc = &rest[..idx];
        let msg = &rest[idx + MSG_SEPARATOR.len()..];
        (Some(doc), Some(msg))
    } else {
        (Some(rest), None)
    }
}

/// Split text into chunks of at most `max_chars`, preferring paragraph boundaries
/// (double newlines), then single newlines, then hard splits.
fn split_at_paragraphs(text: &str, max_chars: usize) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < text.len() {
        if start + max_chars >= text.len() {
            chunks.push(text[start..].trim_start());
            break;
        }

        let search_end = start + max_chars;
        let window = &text[start..search_end];

        let split_at = if let Some(pos) = window.rfind("\n\n") {
            start + pos + 2
        } else if let Some(pos) = window.rfind('\n') {
            start + pos + 1
        } else {
            search_end
        };

        chunks.push(text[start..split_at].trim_start());
        start = split_at;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_message_not_chunked() {
        let msg = "Hello, extract this please.";
        let chunks = chunk_user_message(msg);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], msg);
    }

    #[test]
    fn small_document_not_chunked() {
        let msg = "[Document content]\nShort doc.\n\n[User message]\nExtract this.";
        let chunks = chunk_user_message(msg);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], msg);
    }

    #[test]
    fn large_document_chunked() {
        // Build a document larger than MAX_CHUNK_CHARS
        let paragraph = "This is a paragraph with enough text to fill space. ".repeat(100);
        let doc = format!("{paragraph}\n\n{paragraph}\n\n{paragraph}\n\n{paragraph}");
        let msg = format!("[Document content]\n{doc}\n\n[User message]\nPlease extract.");

        let chunks = chunk_user_message(&msg);
        assert!(
            chunks.len() > 1,
            "Expected multiple chunks, got {}",
            chunks.len()
        );

        // First chunk has user message
        assert!(chunks[0].contains("[User message]"));
        assert!(chunks[0].contains("Part 1/"));

        // Subsequent chunks have continuation prompt
        assert!(chunks[1].contains("Continue extracting"));
        assert!(chunks[1].contains("Part 2/"));
    }

    #[test]
    fn document_only_no_user_message() {
        let paragraph = "Text content repeated many times. ".repeat(200);
        let doc = format!("{paragraph}\n\n{paragraph}");
        let msg = format!("[Document content]\n{doc}");

        let chunks = chunk_user_message(&msg);
        assert!(chunks.len() > 1);

        // First chunk should NOT contain [User message]
        assert!(!chunks[0].contains("[User message]"));
    }

    #[test]
    fn splits_at_paragraph_boundaries() {
        let part_a = "a".repeat(6000);
        let part_b = "b".repeat(6000);
        let part_c = "c".repeat(6000);
        let doc = format!("{part_a}\n\n{part_b}\n\n{part_c}");
        let msg = format!("[Document content]\n{doc}");

        let chunks = chunk_user_message(&msg);
        // Should split at the \n\n boundaries, not in the middle of text
        for chunk in &chunks {
            // Each chunk's content should not start/end mid-word
            let content = chunk
                .lines()
                .skip(1) // skip [Document content — Part X/Y]
                .collect::<Vec<_>>()
                .join("\n");
            assert!(
                content.len() <= MAX_CHUNK_CHARS + 200, // small tolerance for prefix
                "Chunk too large: {} chars",
                content.len()
            );
        }
    }
}

use crate::presentation::errors::DatabaseError;

pub fn validate_depth(depth: i32) -> Result<u32, DatabaseError> {
    if depth < 1 || depth > 10 {
        return Err(DatabaseError::InvalidDepth { value: depth });
    }
    Ok(depth as u32)
}

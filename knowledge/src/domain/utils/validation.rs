use crate::infrastructure::errors::DatabaseError;

pub fn validate_depth(depth: i32) -> Result<u32, DatabaseError> {
    if !(1..=10).contains(&depth) {
        return Err(DatabaseError::InvalidDepth { value: depth });
    }
    Ok(depth.cast_unsigned())
}

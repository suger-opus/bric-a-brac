use rand::RngExt;

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn generate_key() -> String {
    let mut rng = rand::rng();
    let first = LETTERS[rng.random_range(0..52)] as char;
    let rest: String = (0..7)
        .map(|_| BASE62[rng.random_range(0..62)] as char)
        .collect();
    format!("{}{}", first, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key_format() {
        let key = generate_key();

        assert_eq!(key.len(), 8);
        assert!(key.chars().next().unwrap().is_ascii_alphabetic());
        assert!(key.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_generate_key_uniqueness() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for _ in 0..1000 {
            let key = generate_key();
            assert!(set.insert(key), "Duplicate key generated");
        }
    }
}

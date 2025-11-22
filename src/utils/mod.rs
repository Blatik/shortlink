use rand::Rng;
use uuid::Uuid;

// Base62 characters for short code generation
const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Generate a random short code using base62 encoding
pub fn generate_short_code(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..BASE62.len());
            BASE62[idx] as char
        })
        .collect()
}

/// Generate a UUID v4
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Generate an API key
pub fn generate_api_key() -> String {
    format!("sk_{}", Uuid::new_v4().simple())
}

/// Validate URL format
pub fn is_valid_url(url: &str) -> bool {
    match url::Url::parse(url) {
        Ok(parsed) => parsed.scheme() == "http" || parsed.scheme() == "https",
        Err(_) => false,
    }
}

/// Validate custom alias (3-20 alphanumeric characters, hyphens, underscores)
pub fn is_valid_alias(alias: &str) -> bool {
    if alias.len() < 3 || alias.len() > 20 {
        return false;
    }
    alias.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Get current ISO 8601 timestamp
pub fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_code() {
        let code = generate_short_code(6);
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| BASE62.contains(&(c as u8))));
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com/path"));
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("not a url"));
    }

    #[test]
    fn test_is_valid_alias() {
        assert!(is_valid_alias("my-link"));
        assert!(is_valid_alias("test_123"));
        assert!(!is_valid_alias("ab")); // too short
        assert!(!is_valid_alias("this-is-way-too-long-alias")); // too long
        assert!(!is_valid_alias("invalid@alias")); // invalid char
    }
}

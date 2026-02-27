/// Builds a welcome message for a validated name.
pub fn welcome(name: &str) -> Result<String, String> {
    let cleaned = validate_name(name)?;
    Ok(format!("Hello, {cleaned}! Welcome to rust-example."))
}

/// Adds two i32 numbers and returns an overflow-safe result.
pub fn checked_add(a: i32, b: i32) -> Result<i32, String> {
    a.checked_add(b)
        .ok_or_else(|| format!("Overflow while adding {a} and {b}"))
}

fn validate_name(name: &str) -> Result<&str, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c == '-' || c == ' ')
    {
        return Err("Name must only contain letters, spaces, or '-'".to_string());
    }

    Ok(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_contains_name() {
        let msg = welcome("Rustacean").expect("valid name should pass");
        assert!(msg.contains("Rustacean"));
    }

    #[test]
    fn welcome_rejects_empty_name() {
        let err = welcome("  ").expect_err("empty name should fail");
        assert_eq!(err, "Name cannot be empty");
    }

    #[test]
    fn checked_add_works() {
        assert_eq!(checked_add(2, 3).expect("no overflow"), 5);
    }

    #[test]
    fn checked_add_detects_overflow() {
        let err = checked_add(i32::MAX, 1).expect_err("overflow should fail");
        assert!(err.contains("Overflow"));
    }
}

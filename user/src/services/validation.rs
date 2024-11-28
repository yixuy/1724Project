use crate::error::InputError;

/* User input validation utilities */

pub fn validate_username(username: &str) -> Result<(), InputError>{
    // checks username length and format
    if !(3..=20).contains(&username.chars().count()) {
        return Err(InputError::UsernameInvalidLength);
    }
    if !username.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(InputError::UsernameInvalidFormat);
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), InputError> {
    // validates password strength
    if !(8..=64).contains(&password.chars().count()) {
        return Err(InputError::PasswordInvalidLength);
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(InputError::PasswordMissingUppercase);
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(InputError::PasswordMissingLowercase);
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(InputError::PasswordMissingNumber);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_valid() {
        let usernames = vec![
            "abc",
            "Username",
            "1234567890",
            "Username123",
            "Username20Characters",
        ];
        for username in usernames {
            assert!(validate_username(username).is_ok(), "Username should be valid");
        }
    }

    #[test]
    fn test_validate_username_invalid_length() {
        let usernames = vec![
            "",
            "a",
            "B2",
            "UsernameIsTooLong1234",
        ];

        for username in usernames {
            let res = validate_username(username);
            assert!(res.is_err(), "Username should be invalid");
        }
    }

    #[test]
    fn test_validate_username_invalid_format() {
        let usernames = vec![
            "s p a c e",
            "abc123!",
            "@some",
            "username#1",
            "money$",
            "中文名",
        ];

        for username in usernames {
            let res = validate_username(username);
            assert!(res.is_err(), "Username should be invalid");
        }
    }

    #[test]
    fn test_validate_password_valid() {
        let passwords = vec![
            "Password123",
            "!AdminPass0",
            "PasswordCanBeVeryLong0123456789~`!@#$%^&*()_+-={}[]|:;<>,./?is64",
        ];
        for password in passwords {
            assert!(validate_password(password).is_ok(), "Password should be valid");
        }
    }

    #[test]
    fn test_validate_password_invalid_length() {
        let passwords = vec![
            "",
            "Pw12345",
            "PasswordIsToLong0123456789~`!@#$%^&*()_+-={}[]|:;<>,./?isLonger65",
        ];
        for password in passwords {
            let res = validate_password(password);
            assert!(res.is_err(), "Password should be invalid");
        }
    }

    #[test]
    fn test_validate_password_invalid_format() {
        let passwords = vec![
            "all-lower",
            "ALL_CAPITAL",
            "0123456789",
            "PasswordWithoutNumber",
            "PW123456",
            "password123",
            "!@#$%^&*-+_="
        ];
        for password in passwords {
            assert!(validate_password(password).is_err(), "Password should be invalid");
        }
    }
}
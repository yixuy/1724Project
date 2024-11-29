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

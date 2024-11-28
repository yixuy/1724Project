use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng, PasswordHash},
    Argon2,
};
use crate::error::PasswordError;

/* Password hashing utilities */

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    // hashes a password using argon2
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2.hash_password(password.as_bytes(), &salt)
        .map(|hashed| hashed.to_string())
        .map_err(|err| {
            eprintln!("Failed to hash password: {:?}", err);
            PasswordError::HashingError
        })
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<(), PasswordError> {
    // verifies a hashed password
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|err| {
            eprintln!("Failed to parse hashed password: {:?}", err);
            PasswordError::VerificationError
        })?;
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| {
            if err == argon2::password_hash::Error::Password {
                PasswordError::PasswordInvalid
            } else {
                eprintln!("Failed to verify password: {:?}", err);
                PasswordError::VerificationError
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "Password123";
        let hashed = hash_password(password);
        assert!(hashed.is_ok(), "Hash should succeed");
        let result = verify_password(password, &hashed.unwrap());
        assert!(result.is_ok(), "Password should match");
    }

    #[test]
    fn test_password_mismatch() {
        let password = "Password123";
        let hashed = hash_password(password).unwrap();
        let wrong_password = "WrongPassword123";
        let result = verify_password(wrong_password, &hashed);
        assert!(result.is_err(), "Wrong password should fail");
    }
}

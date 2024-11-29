use crate::error::PasswordError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/* Password hashing utilities */

pub fn hash_password(password: String) -> Result<String, PasswordError> {
    // hashes a password using argon2
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hashed| hashed.to_string())
        .map_err(|err| {
            eprintln!("Failed to hash password: {:?}", err);
            PasswordError::HashingError
        })
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, PasswordError> {
    // verifies a hashed password
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|err| {
        eprintln!("Failed to parse hashed password: {:?}", err);
        PasswordError::VerificationError
    })?;
    let argon2 = Argon2::default();
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map(|_| true)
        .map_err(|err| {
            if err == argon2::password_hash::Error::Password {
                PasswordError::PasswordInvalid
            } else {
                eprintln!("Failed to verify password: {:?}", err);
                PasswordError::VerificationError
            }
        })
}

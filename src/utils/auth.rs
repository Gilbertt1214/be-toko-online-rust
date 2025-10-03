use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

/// Hash password dengan argon2 (salt random)
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Gagal hash password: {}", e))
}

/// Verify password dengan hash yang tersimpan
pub fn verify_password(hashed: &str, password: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hashed)
        .map_err(|e| format!("Invalid hash format: {}", e))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
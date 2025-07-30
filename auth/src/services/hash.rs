use argon2::password_hash::{rand_core::OsRng, Error as PasswordHashError, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub struct Hasher {
    argon2: Argon2<'static>,
}

impl Hasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub fn hash(&self, password: &str) -> Result<String, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);
        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }

    pub fn verify(&self, password: &str, hashed: &str) -> Result<bool, PasswordHashError> {
        let parsed = PasswordHash::new(hashed)?;
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}

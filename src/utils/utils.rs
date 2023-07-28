use bcrypt::{hash, verify, DEFAULT_COST};
use std::io::Error as IoError;
use std::str;

pub fn change_to_utf8(data: &[u8]) -> Result<String, IoError> {
    if let Ok(utf8_string) = std::str::from_utf8(&data) {
        println!("Decoded string: {}", utf8_string);
        Ok(utf8_string.into())
    } else {
        println!("Failed to decode the byte string as UTF-8.");
        Err(IoError::new(
            std::io::ErrorKind::InvalidData,
            "Failed to decode the byte string as UTF-8.",
        ))
    }
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(input_password: &str, hashed_password: &str) -> bool {
    match verify(input_password, hashed_password) {
        Ok(valid) => valid,
        Err(_) => false,
    }
}

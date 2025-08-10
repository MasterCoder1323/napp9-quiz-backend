use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use fastrand::Rng;
use base64::{engine::general_purpose, Engine as _};

const ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16;
const HASH_LEN: usize = 32;

pub fn hash_password(password: &str) -> String {
    let mut salt = [0u8; SALT_LEN];
    Rng::new().fill(&mut salt);

    let mut hash = [0u8; HASH_LEN];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &salt,
        ITERATIONS,
        &mut hash,
    );

    // Store salt + hash together, base64 encoded
    let mut combined = Vec::with_capacity(SALT_LEN + HASH_LEN);
    combined.extend_from_slice(&salt);
    combined.extend_from_slice(&hash);

    general_purpose::STANDARD.encode(&combined)
}

pub fn verify_password(password: &str, stored: &str) -> bool {
    let decoded = match general_purpose::STANDARD.decode(stored) {
        Ok(d) => d,
        Err(_) => return false,
    };

    if decoded.len() != SALT_LEN + HASH_LEN {
        return false;
    }

    let salt = &decoded[..SALT_LEN];
    let hash = &decoded[SALT_LEN..];

    let mut computed_hash = [0u8; HASH_LEN];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        salt,
        ITERATIONS,
        &mut computed_hash,
    );

    // Constant time comparison
    subtle::ConstantTimeEq::constant_time_eq(hash, &computed_hash)
}

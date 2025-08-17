use pbkdf2::pbkdf2;
use fastrand::Rng;
use base64::{engine::general_purpose, Engine as _};
use subtle::ConstantTimeEq;
use sha2::Sha256;
use hmac::Hmac; // ← Add this
type HmacSha256 = Hmac<Sha256>; // ← Type alias for clarity

const ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16;
const HASH_LEN: usize = 32;

pub fn hash_password(password: &str) -> String {
    let mut salt = [0u8; SALT_LEN];
    Rng::new().fill(&mut salt);

    let mut hash = [0u8; HASH_LEN];
    pbkdf2::<HmacSha256>( // ← Use HmacSha256 here
        password.as_bytes(),
        &salt,
        ITERATIONS,
        &mut hash,
    );

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
    pbkdf2::<HmacSha256>( // ← Same here
        password.as_bytes(),
        salt,
        ITERATIONS,
        &mut computed_hash,
    );

    hash.ct_eq(&computed_hash).into()
}

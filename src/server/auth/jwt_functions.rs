use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub username: String,
    pub exp: usize,
}

pub fn generate_expiration(minutes_from_now: u64) -> usize {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time went backwards");
    (now.as_secs() + (minutes_from_now * 60)) as usize
}

/// Helper function to grab the secret from the environment
pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env or system environment")
}

pub fn create_jwt(username: &str, id: &str) -> Option<String> {
    let secret = get_jwt_secret(); // Fetch the secret here

    let claims = Claims {
        id: id.to_owned(),
        username: username.to_owned(),
        exp: generate_expiration(60),
    };

    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&header, &claims, &key).ok()
}

pub fn verify_jwt_custom(token: &str) -> Option<Claims> {
    let secret = get_jwt_secret(); // Fetch the secret here

    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &key, &validation) {
        Ok(token_data) => Some(token_data.claims),
        Err(_) => None,
    }
}

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET: &[u8] = b"SECRET_KEY"; // ganti dengan env variable

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // biasanya email
    pub exp: usize,
}

pub fn create_jwt(email: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp();

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn verify_jwt(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default()).is_ok()
}

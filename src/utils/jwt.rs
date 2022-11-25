use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}

fn read_key() -> String {
    std::env::var("JWT_KEY").unwrap()
}

pub fn verify(token: String) -> Option<String> {
    let key = read_key();
    let key = key.as_bytes();

    let decoding_key = DecodingKey::from_secret(key);

    let validation = Validation::new(Algorithm::HS256);

    let claims = jsonwebtoken::decode::<Claims>(token.as_str(), &decoding_key, &validation);

    match claims {
        Ok(claims) => Some(claims.claims.user_id),
        Err(_) => None,
    }
}

pub fn sign(exp: usize, user_id: String) -> String {
    let key = read_key();
    let key = key.as_bytes();

    let data = Claims { user_id, exp };

    let header = Header::new(Algorithm::HS256);

    jsonwebtoken::encode::<Claims>(&header, &data, &EncodingKey::from_secret(key))
        .unwrap_or("".into())
}

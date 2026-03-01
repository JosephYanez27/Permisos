use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id_usuario: i32,
    pub id_perfil: i32,
    pub exp: usize,
}

pub fn generate_jwt(id_usuario: i32, id_perfil: i32) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(2))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        id_usuario,
        id_perfil,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret_key".as_ref()),
    ).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret_key".as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
use jsonwebtoken::{decode, Algorithm, Header, EncodingKey, DecodingKey, Validation};
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, HttpResponse};
use crate::utils::CowStr;
use crate::config::CONFIG;

static KEYS: LazyLock<(EncodingKey, DecodingKey)> = LazyLock::new(|| {
    let config = CONFIG.read().unwrap();
    (EncodingKey::from_secret(config.secrets.jwt.as_bytes()), DecodingKey::from_secret(config.secrets.jwt.as_bytes()))
});
static VALIDATION: LazyLock<Validation> = LazyLock::new(|| Validation::new(Algorithm::HS256));

#[derive(Deserialize, Serialize, Clone)]
struct AuthToken {
    sub: CowStr,
}

enum AuthStatus {
    Authenticated(AuthToken),
    Unauthenticated,
    InvalidToken,
}

impl AuthStatus {
    pub fn from_req(req: HttpRequest) -> AuthStatus {
        if let Some(cookie) = req.cookie("token") {
            if let Ok(token) = decode::<AuthToken>(cookie.value(), &KEYS.1, &VALIDATION) {
                return AuthStatus::Authenticated(token.claims);
            }
            return AuthStatus::InvalidToken;
        }
        AuthStatus::Unauthenticated
    }

    pub fn handle(self, res: &mut HttpResponse) -> bool {
        true
    }
}

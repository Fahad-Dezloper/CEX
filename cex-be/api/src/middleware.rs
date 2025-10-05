use poem::Request;
use crate::auth_service::{AuthService, Claims};

pub fn extract_claims(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}

pub fn verify_token(request: &mut Request) -> Result<&Claims, String> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    if let Some(token) = auth_header {
        let auth_service = AuthService::new();
        
        match auth_service.verify_token(token) {
            Ok(claims) => {
                request.extensions_mut().insert(claims);
                Ok(request.extensions().get::<Claims>().unwrap())
            }
            Err(_) => Err("Invalid or expired token".to_string())
        }
    } else {
        Err("Missing or invalid Authorization header".to_string())
    }
}

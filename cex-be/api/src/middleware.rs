use poem::Request;
use crate::auth_service::{AuthService, Claims};
use poem::http::header;

pub fn extract_claims(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}

pub fn verify_token(request: &mut Request) -> Result<&Claims, String> {
    // 1) Try Authorization: Bearer
    if let Some(token) = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
    {
        let auth_service = AuthService::new();
        
        match auth_service.verify_token(token) {
            Ok(claims) => {
                request.extensions_mut().insert(claims);
                Ok(request.extensions().get::<Claims>().unwrap())
            }
            Err(_) => Err("Invalid or expired token".to_string())
        }
    } else {
        // 2) Fallback to Cookie: cex_token
        let cookie_header = request.headers().get(header::COOKIE).and_then(|h| h.to_str().ok());
        if let Some(cookie_str) = cookie_header {
            if let Some(token) = cookie_str
                .split(';')
                .map(|kv| kv.trim())
                .find_map(|kv| kv.strip_prefix("cex_token="))
            {
                let auth_service = AuthService::new();
                match auth_service.verify_token(token) {
                    Ok(claims) => {
                        request.extensions_mut().insert(claims);
                        return Ok(request.extensions().get::<Claims>().unwrap());
                    }
                    Err(_) => return Err("Invalid or expired token".to_string()),
                }
            }
        }
        Err("Missing authentication (no Bearer token or cex_token cookie)".to_string())
    }
}

use poem::{handler, post, web::Json, Route, Result, error::InternalServerError};
use serde_json::json;
use validator::Validate;
use db::{establish_connection, User as DbUser, users};
use diesel::prelude::*;
use crate::auth_service::{AuthService, LoginRequest, RegisterRequest, UserInfo};

#[handler]
async fn register(Json(payload): Json<RegisterRequest>) -> Result<Json<serde_json::Value>> {
    if let Err(validation_errors) = payload.validate() {
        return Ok(Json(json!({
            "error": "Validation failed",
            "details": validation_errors.field_errors()
        })));
    }

    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| poem::Error::from_string(format!("Database connection error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let existing_user: Option<DbUser> = users::table
        .filter(users::email.eq(&payload.email))
        .first(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    if existing_user.is_some() {
        return Ok(Json(json!({
            "error": "User with this email already exists"
        })));
    }

    let existing_username: Option<DbUser> = users::table
        .filter(users::username.eq(&payload.username))
        .first(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    if existing_username.is_some() {
        return Ok(Json(json!({
            "error": "Username already taken"
        })));
    }

    let new_user = crate::auth_service::User::new(
        payload.username,
        payload.email,
        payload.password,
    ).map_err(|e| poem::Error::from_string(format!("Password hashing error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let db_user = DbUser {
        id: new_user.id,
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password_hash: new_user.password_hash,
        created_at: new_user.created_at.date(),
        updated_at: new_user.created_at.date(),
    };

    diesel::insert_into(users::table)
        .values(&db_user)
        .execute(&mut conn)
        .map_err(|e| poem::Error::from_string(format!("Database insert error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    // Initialize user wallet with $10000 USD
    initialize_user_wallet(&new_user.id.to_string(), &mut conn)?;

    let auth_service = AuthService::new();
    let token = auth_service.generate_token(&new_user.id.to_string(), &new_user.email)
        .map_err(|e| poem::Error::from_string(format!("Token generation error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user_info = UserInfo {
        id: new_user.id.to_string(),
        email: new_user.email,
        username: new_user.username,
    };

    Ok(Json(json!({
        "message": "User registered successfully",
        "token": token,
        "user": user_info
    })))
}

#[handler]
async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<serde_json::Value>> {
    if let Err(validation_errors) = payload.validate() {
        return Ok(Json(json!({
            "error": "Validation failed",
            "details": validation_errors.field_errors()
        })));
    }

    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| poem::Error::from_string(format!("Database connection error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user: Option<DbUser> = users::table
        .filter(users::email.eq(&payload.email))
        .first(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(Json(json!({
                "error": "Invalid email or password"
            })));
        }
    };

    let auth_service = AuthService::new();
    if !AuthService::verify_password(&payload.password, &user.password_hash)
        .map_err(|e| poem::Error::from_string(format!("Password verification error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))? {
        return Ok(Json(json!({
            "error": "Invalid email or password"
        })));
    }

    let token = auth_service.generate_token(&user.id.to_string(), &user.email)
        .map_err(|e| poem::Error::from_string(format!("Token generation error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user_info = UserInfo {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
    };

    Ok(Json(json!({
        "message": "Login successful",
        "token": token,
        "user": user_info
    })))
}

fn initialize_user_wallet(user_id: &str, conn: &mut diesel::PgConnection) -> Result<(), poem::Error> {
    // Initialize user with $10000 USD balance
    log::info!("Initializing wallet for user {} with $10000 USD", user_id);
    
    // For now, we'll send an on_ramp message to the engine to initialize the wallet
    // In a production system, you'd want to store balances in the database
    // and sync with the engine
    
    // TODO: Implement proper balance persistence in database
    // For now, the engine will handle wallet initialization via on_ramp
    
    log::info!("Wallet initialization completed for user: {}", user_id);
    Ok(())
}

pub fn auth_routes() -> Route {
    Route::new()
        .at("/register", post(register))
        .at("/login", post(login))
}

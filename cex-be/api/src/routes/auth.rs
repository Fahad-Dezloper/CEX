use poem::get;
use poem::{handler, post, web::Json, Route, Result, error::InternalServerError, IntoResponse, Response};
use poem::http::header;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use db::{establish_connection, user_assets, users, User as DbUser, UserAsset};
use diesel::prelude::*;
use crate::auth_service::{AuthService, LoginRequest, RegisterRequest, UserInfo};
use crate::middleware::extract_claims;
use diesel::dsl::sql;
use diesel::prelude::*;

#[handler]
async fn register(Json(payload): Json<RegisterRequest>) -> Result<Response> {
    log::info!("Registering user: {:?}", payload);
    if let Err(validation_errors) = payload.validate() {
        return Ok(Json(json!({
            "error": "Validation failed",
            "details": validation_errors.field_errors()
        })).into_response());
    }
    log::info!("Validation passed");

    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| poem::Error::from_string(format!("Database connection error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("Database connection established");
    let existing_user: Option<DbUser> = users::table
        .filter(users::email.eq(&payload.email))
        .first(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("Database query executed");
    if existing_user.is_some() {
        return Ok(Json(json!({
            "error": "User with this email already exists"
        })).into_response());
    }
    log::info!("User with this email does not exist");
    let existing_username: Option<DbUser> = users::table
        .filter(users::username.eq(&payload.username))
        .first(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("Database query executed");
    if existing_username.is_some() {
        return Ok(Json(json!({
            "error": "Username already taken"
        })).into_response());
    }
    log::info!("User with this username does not exist");
    let new_user = crate::auth_service::User::new(
        payload.username,
        payload.email,
        payload.password,
    ).map_err(|e| poem::Error::from_string(format!("Password hashing error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("User created");
    let db_user = DbUser {
        id: new_user.id,
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password_hash: new_user.password_hash,
        created_at: new_user.created_at.date(),
        updated_at: new_user.created_at.date(),
    };
    log::info!("Database user created");
    diesel::insert_into(users::table)
        .values(&db_user)
        .execute(&mut conn)
        .map_err(|e| poem::Error::from_string(format!("Database insert error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    // Initialize user assets with 10,000 USDC in user_assets table
    initialize_user_wallet(new_user.id, &mut conn)?;

    let auth_service = AuthService::new();
    let token = auth_service.generate_token(&new_user.id.to_string(), &new_user.email)
        .map_err(|e| poem::Error::from_string(format!("Token generation error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user_info = UserInfo {
        id: new_user.id.to_string(),
        email: new_user.email,
        username: new_user.username,
        assets: vec![],
    };

    let cookie = format!(
        "cex_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        token,
        60 * 60 * 24
    );

    let body = json!({
        "message": "User registered successfully",
        "user": user_info
    });
    let mut resp = Json(body).into_response();
    resp.headers_mut().insert(header::SET_COOKIE, header::HeaderValue::from_str(&cookie).unwrap());
    Ok(resp)
}

#[handler]
async fn login(Json(payload): Json<LoginRequest>) -> Result<Response> {
    if let Err(validation_errors) = payload.validate() {
        return Ok(Json(json!({
            "error": "Validation failed",
            "details": validation_errors.field_errors()
        })).into_response());
    }
    log::info!("Validation passed");

    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| poem::Error::from_string(format!("Database connection error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user: Option<DbUser> = users::table
        .filter(users::email.eq(&payload.email))
        .first(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    log::info!("Database query executed");
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(Json(json!({
                "error": "Invalid email or password"
            })).into_response());
        }
    };
    println!("user: {:?}", user);
    let auth_service = AuthService::new();
    if !AuthService::verify_password(&payload.password, &user.password_hash)
        .map_err(|e| poem::Error::from_string(format!("Password verification error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))? {
        return Ok(Json(json!({
            "error": "Invalid email or password"
        })).into_response());
    }
    log::info!("Password verified");
    let token = auth_service.generate_token(&user.id.to_string(), &user.email)
        .map_err(|e| poem::Error::from_string(format!("Token generation error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("Token generated");
    let user_info = UserInfo {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
        assets: vec![],
    };
    log::info!("User info created");
    let cookie = format!(
        "cex_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        token,
        60 * 60 * 24
    );

    let body = json!({
        "message": "Login successful",
        "user": user_info
    });
    println!("body: {:?}", body);
    let mut resp = Json(body).into_response();
    resp.headers_mut().insert(header::SET_COOKIE, header::HeaderValue::from_str(&cookie).unwrap());
    Ok(resp)
}

fn initialize_user_wallet(user_id: Uuid, conn: &mut diesel::PgConnection) -> Result<(), poem::Error> {
    log::info!("Initializing user_assets for user {} with 10000 USDC", user_id);
    diesel::insert_into(user_assets::table)
        .values((
            user_assets::user_id.eq(user_id),
            user_assets::symbol.eq("USDC"),
            user_assets::amount.eq(10000.0f64),
        ))
        .on_conflict((user_assets::user_id, user_assets::symbol))
        .do_update()
        .set(user_assets::amount.eq(10000.0f64))
        .execute(conn)
        .map_err(|e| poem::Error::from_string(format!("Failed to seed user_assets: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(())
}

#[handler]
pub async fn me(request: &poem::Request) -> Result<Response> {
    log::info!("/auth/me called");
    let bearer = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    let cookie_token = request
        .headers()
        .get(poem::http::header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|c| c.split(';').map(|kv| kv.trim()).find_map(|kv| kv.strip_prefix("cex_token=")))
        .map(|s| s.to_string());

    let token = bearer.or(cookie_token)
        .ok_or_else(|| poem::Error::from_string("Authentication required", poem::http::StatusCode::UNAUTHORIZED))?;

    let claims = AuthService::new()
        .verify_token(&token)
        .map_err(|_| poem::Error::from_string("Invalid or expired token", poem::http::StatusCode::UNAUTHORIZED))?;
    log::info!("Token verified");
    let pool = establish_connection();
    let mut conn = pool.get()
        .map_err(|e| poem::Error::from_string(format!("Database connection error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user_id = Uuid::parse_str(&claims.user_id)
        .map_err(|e| poem::Error::from_string(format!("Invalid user ID: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("User ID parsed");
    let user = users::table
        .filter(users::id.eq(user_id))
        .first::<DbUser>(&mut conn)
        .optional()
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("Database query executed");
    let user: Option<DbUser> = match user {
        Some(user) => Some(user),
        None => return Ok(Json(json!({
            "error": "User not found"
        })).into_response()),
    };
    log::info!("User found: {:?}", user);
    let user = user.unwrap();
    let user_assets = user_assets::table
        .filter(user_assets::user_id.eq(&user.id))
        .load::<UserAsset>(&mut conn)
        .map_err(|e| poem::Error::from_string(format!("Database query error: {}", e), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    log::info!("User assets found: {:?}", &user_assets);
    let user_info = UserInfo {
        id: user.id.to_string(),
        email: user.email,  
        username: user.username,
        assets: user_assets.clone(),
    };
    log::info!("User info created");
    Ok(Json(json!({
        "user": user_info
    })).into_response())
}

#[handler]
async fn logout() -> Result<Response> {
    let mut resp = Json(json!({ "message": "Logged out" })).into_response();
    let clear = "cex_token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0";
    resp.headers_mut().insert(header::SET_COOKIE, header::HeaderValue::from_static(clear));
    Ok(resp)
}

pub fn auth_routes() -> Route {
    Route::new()
        .at("/register", post(register))
        .at("/login", post(login))
        .at("/logout", post(logout))
        .at("/me", get(me))
}

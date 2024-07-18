use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_client_ip::SecureClientIp;
use serde::Deserialize;

use blazebooru_models::local as lm;
use blazebooru_models::view as vm;

use crate::auth::{AuthClaims, JwtClaims, SessionClaims};
use crate::server::api::auth::LoginResponse;
use crate::server::api::Authorized;
use crate::server::{ApiError, BlazeBooruServer};

#[derive(Debug, Deserialize)]
struct RegisterUserRequest {
    name: String,
    password: String,
}

pub fn router() -> Router<Arc<BlazeBooruServer>> {
    Router::new()
        .route("/profile", get(get_user_profile))
        .route("/register", post(register_user))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn get_user_profile(
    State(server): State<Arc<BlazeBooruServer>>,
    auth: Authorized,
) -> Result<Json<vm::User>, ApiError> {
    let user = server.core.get_user_profile(auth.claims.user_id).await?;

    Ok(Json(user.ok_or(ApiError::NotFound)?))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn register_user(
    State(server): State<Arc<BlazeBooruServer>>,
    SecureClientIp(ip): SecureClientIp,
    Json(req): Json<RegisterUserRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    if !server.config.allow_registration {
        return Err(ApiError::Forbidden);
    }

    let user = lm::NewUser {
        name: req.name.into(),
        password: req.password.into(),
    };

    let user_id = server.core.create_user(user).await?;

    let claims = AuthClaims { user_id };

    let lm::CreateRefreshTokenResult {
        token: refresh_token,
        session,
    } = server.core.create_refresh_token(user_id, ip).await?;
    let claims = SessionClaims { session, claims };

    let claims = JwtClaims::short(claims);
    let exp = claims.exp;
    let access_token = server.auth.generate_token(&claims)?;

    Ok(Json(LoginResponse {
        access_token,
        exp,
        refresh_token,
    }))
}

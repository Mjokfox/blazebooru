use std::sync::Arc;

use anyhow::Context;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_client_ip::ClientIp;
use serde::Deserialize;

use blazebooru_models::local as lm;
use blazebooru_models::view as vm;

use crate::auth::{AuthClaims, JwtClaims, SessionClaims};
use crate::server::api::Authorized;
use crate::server::api::auth::LoginResponse;
use crate::server::{ApiError, BlazeBooruServer};

#[derive(Debug, Deserialize)]
struct RegisterUserRequest {
    name: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct PublicProfileUserRequest {
    name: String
}

pub fn router() -> Router<Arc<BlazeBooruServer>> {
    Router::new()
        .route("/{id}", post(update_user).delete(delete_user))
        .route("/profile", get(get_user_profile))
        .route("/pubprofile", get(get_public_user_profile))
        .route("/register", post(register_user))
        .route("/all", get(get_all_users))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn update_user(
    State(server): State<Arc<BlazeBooruServer>>,
    auth: Authorized,
    Path(id): Path<i32>,
    Json(mut req): Json<vm::UpdateUser>,
) -> Result<(), ApiError> {
    let user = server
        .core.get_user_profile(auth.claims.user_id)
        .await?
        .unwrap();

    if id != user.id && user.rank <= 0 {
        return Err(ApiError::Unauthorized);
    }

    if req.rank.is_some() {
        if user.rank <= 0 { // TODO define ranks
            req.rank = None;
        }
    }

    let post = server
        .core
        .update_user(id, req)
        .await
        .context("Error updating user")?;

    if !post {
        return Err(ApiError::NotFound);
    }

    Ok(())
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn delete_user(
    State(server): State<Arc<BlazeBooruServer>>,
    auth: Authorized,
    Path(id): Path<i32>,
) -> Result<(), ApiError> {
    let user = server
        .core.get_user_profile(auth.claims.user_id)
        .await?;
    
    if user.unwrap().rank <= 0 { // TODO define ranks
        return Err(ApiError::Unauthorized);
    }
    
    let success = server
        .core
        .delete_user(id)
        .await
        .context("Error deleting user")?;

    if !success {
        return Err(ApiError::NotFound);
    }

    server.core.logout(auth.session).await?;

    Ok(())
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
async fn get_public_user_profile(
    State(server): State<Arc<BlazeBooruServer>>,
    Query(req): Query<PublicProfileUserRequest>,
) -> Result<Json<vm::PublicUser>, ApiError> {
    let user = server.core.get_public_user_profile(&req.name).await?;

    Ok(Json(user.ok_or(ApiError::NotFound)?))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn get_all_users(
    State(server): State<Arc<BlazeBooruServer>>
) -> Result<Json<Vec<vm::PublicUser>>, ApiError> {
    let users = server.core.get_all_users().await?;

    Ok(Json(users))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn register_user(
    State(server): State<Arc<BlazeBooruServer>>,
    ClientIp(ip): ClientIp,
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

use std::sync::Arc;

use anyhow::Context;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_client_ip::ClientIp;

use blazebooru_models::view as vm;

use crate::server::api::Authorized;
use crate::server::{ApiError, BlazeBooruServer};

pub fn router() -> Router<Arc<BlazeBooruServer>> {
    Router::new()
        .route("/n/{name}", get(get_wiki_page).post(update_wiki_page).delete(delete_wiki_page))
        .route("/list", get(list_wiki_pages))
        .route("/new", post(create_wiki_page))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn create_wiki_page(
    State(server): State<Arc<BlazeBooruServer>>,
    auth: Authorized,
    ClientIp(user_ip): ClientIp,
    Json(req): Json<vm::NewWikiPage>,
) -> Result<Json<vm::WikiPage>, ApiError> {
    if req.title.len() < 1 {
        return Err(ApiError::BadRequest);
    }
    let new_wiki_page = server.core.create_wiki_page(auth.claims.user_id, user_ip, req).await?;

    Ok(Json(new_wiki_page))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn get_wiki_page(
    State(server): State<Arc<BlazeBooruServer>>,
    Path(name): Path<String>
) -> Result<Json<vm::WikiPage>, ApiError> {
    let page = server
        .core
        .get_wiki_page_by_name(name)
        .await;

    match page {
        Ok(p) => Ok(Json(p.ok_or(ApiError::NotFound)?)),
        Err(_) => Err(ApiError::Forbidden),
    }
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn list_wiki_pages(
    State(server): State<Arc<BlazeBooruServer>>
) -> Result<Json<Vec<vm::WikiPage>>, ApiError> {
    let pages = server
        .core
        .get_all_wiki_pages()
        .await?;

    Ok(Json(pages))
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn update_wiki_page(
    State(server): State<Arc<BlazeBooruServer>>,
    auth: Authorized,
    ClientIp(user_ip): ClientIp,
    Path(_): Path<String>,
    Json(req): Json<vm::UpdateWikiPage>,
) -> Result<(), ApiError> { // TODO implement rank permissions
    let wiki_page = server
        .core
        .update_wiki_page(auth.claims.user_id, user_ip, req)
        .await
        .context("Error updating wiki page")?;

    if !wiki_page {
        return Err(ApiError::NotFound);
    }

    Ok(())
}

#[axum::debug_handler(state = Arc<BlazeBooruServer>)]
async fn delete_wiki_page(
    State(server): State<Arc<BlazeBooruServer>>,
    auth: Authorized,
    Path(name): Path<String>
) -> Result<(), ApiError> {
    let user = server
        .core.get_user_profile(auth.claims.user_id)
        .await?;
    
    if user.unwrap().rank <= 0 { // TODO define ranks
        return Err(ApiError::Unauthorized);
    }
    
    let success = server
        .core
        .delete_wiki_page(name)
        .await
        .context("Error deleting wiki")?;

    if !success {
        return Err(ApiError::NotFound);
    }

    Ok(())
}



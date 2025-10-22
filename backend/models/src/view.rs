use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub user_id: i32,
    pub user_name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub filename: String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub hash: String,
    pub ext: String,
    pub tn_ext: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,

    pub add_tags: Vec<String>,
    pub remove_tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub alias_of_tag: Option<String>,
    pub aliases: Vec<String>,
    pub implied_tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTag {
    #[serde(default)]
    pub add_aliases: Vec<String>,
    #[serde(default)]
    pub remove_aliases: Vec<String>,
    #[serde(default)]
    pub add_implied_tags: Vec<String>,
    #[serde(default)]
    pub remove_implied_tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Comment {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct NewPostComment {
    pub comment: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub rank: i16,
    pub biography: Option<String>,
    pub css: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub rank: i16,
    pub biography: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub rank: Option<i16>,
    pub biography: Option<String>,
    pub css: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    pub no: i32,
    pub start_id: i32,
}

#[derive(Debug, Serialize)]
pub struct Config {
    pub max_image_size: usize,
    pub require_login: bool,
    pub allow_registration: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiPage {
    pub id: i32,
    pub creator_id: i32,
    pub creator_name: String,
    pub title: String,
    pub body: String,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub revision: i32,
    pub updated_at: DateTime<Utc>,
    pub updater_id: Option<i32>,
    pub updater_name: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWikiPage {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub locked: bool,
    pub reason: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWikiPage {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub locked: bool,
    pub deleted: bool,
    pub reason: String
}

use std::net::IpAddr;

use anyhow::Context;

use crate::{PgStore, StoreError, models as dbm};

impl PgStore {
    pub async fn create_wiki_page(&self, user_id: i32, user_ip: IpAddr, page: &dbm::NewWikiPage) -> Result<dbm::WikiPage, StoreError> {
        let wiki_page = sqlx::query_as_unchecked!(dbm::WikiPage, r#"SELECT * FROM create_wiki($1, $2, $3);"#, page, user_id, user_ip)
            .fetch_one(&self.pool)
            .await
            .context("Error creating wiki page in database")?;

        Ok(wiki_page)
    }

    pub async fn update_wiki_page(&self, user_id: i32, user_ip: IpAddr, page: &dbm::UpdateWikiPage) -> Result<bool, StoreError> {
        let success = sqlx::query_scalar_unchecked!(r#"SELECT update_wiki($1, $2, $3);"#, page, user_id, user_ip)
            .fetch_one(&self.pool)
            .await
            .context("Error updating wiki page in database")?;
        
        Ok(success.unwrap())
    }

    pub async fn delete_wiki_page(&self, name: String) -> Result<bool, StoreError> {
        let success = sqlx::query_scalar_unchecked!(r#"SELECT delete_wiki($1);"#, name)
        .fetch_one(&self.pool)
        .await
        .context("Error deleting wiki page in database")?;
        
        Ok(success.unwrap())
    }

    pub async fn get_wiki_page(&self, id: i32) -> Result<Option<dbm::WikiPage>, StoreError> {
        let wiki_page = sqlx::query_as!(dbm::WikiPage, r#"SELECT * FROM view_wiki WHERE id = $1;"#, id)
            .fetch_optional(&self.pool)
            .await
            .context("Error getting wiki page from database")?;

        if let Some(page) = &wiki_page {
            if page.deleted.unwrap_or(false) {
                return Err(StoreError::Anyhow(anyhow::anyhow!(
                    "Wiki page with id {} is deleted",
                    id
                )));
            }
        }

        Ok(wiki_page)
    }

    pub async fn get_wiki_page_by_name(&self, name: String) -> Result<Option<dbm::WikiPage>, StoreError> {
        let wiki_page = sqlx::query_as!(dbm::WikiPage, r#"SELECT * FROM view_wiki WHERE title = $1;"#, name)
            .fetch_optional(&self.pool)
            .await
            .context("Error getting wiki page from database")?;

        if let Some(page) = &wiki_page {
            if page.deleted.unwrap_or(false) {
                return Err(StoreError::Anyhow(anyhow::anyhow!(
                    "Wiki page {} is deleted",
                    name
                )));
            }
        }

        Ok(wiki_page)
    }

    pub async fn get_all_wiki_pages(&self) -> Result<Vec<dbm::WikiPage>, StoreError> {
        let pages = sqlx::query_as!(dbm::WikiPage, r#"SELECT * FROM view_wiki WHERE NOT deleted ORDER BY id ASC"#)
            .fetch_all(&self.pool)
            .await
            .context("Error getting wiki pages from database")?;

        Ok(pages)
    }
}

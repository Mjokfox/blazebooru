use std::net::IpAddr;

use anyhow::Context;
use uuid::Uuid;

use crate::{
    PgStore,
    models::{CreateRefreshTokenResult, RefreshRefreshTokenResult},
};

impl PgStore {
    pub async fn create_refresh_token(
        &self,
        user_id: i32,
        ip: IpAddr,
    ) -> Result<CreateRefreshTokenResult, anyhow::Error> {
        let token = sqlx::query_as_unchecked!(
            CreateRefreshTokenResult,
            r#"SELECT * FROM create_refresh_token($1, $2);"#,
            user_id,
            ip
        )
        .fetch_one(&self.pool)
        .await
        .context("Error creating refresh token")?;

        Ok(token)
    }

    pub async fn invalidate_session(&self, session: i64) -> Result<(), anyhow::Error> {
        sqlx::query_as_unchecked!(
            CreateRefreshTokenResult,
            r#"SELECT * FROM invalidate_session($1);"#,
            session
        )
        .execute(&self.pool)
        .await
        .context("Error invalidating session")?;

        Ok(())
    }

    pub async fn refresh_refresh_token(
        &self,
        token: Uuid,
        ip: IpAddr,
    ) -> Result<RefreshRefreshTokenResult, anyhow::Error> {
        let result = sqlx::query_as_unchecked!(
            RefreshRefreshTokenResult,
            r#"SELECT * FROM refresh_refresh_token($1, $2);"#,
            token,
            ip
        )
        .fetch_optional(&self.pool)
        .await?
        .context("Error refreshing refresh token")?;

        Ok(result)
    }
}

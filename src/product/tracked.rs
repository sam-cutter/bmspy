use sqlx::{Error as SqlxError, PgPool};

use super::Product;

impl Product {
    pub async fn tracked(&self, pool: &PgPool) -> Result<bool, SqlxError> {
        let tracked = match sqlx::query!(
            r"
            SELECT EXISTS (
                SELECT 1
                FROM products
                WHERE uuid = $1
            )
            ",
            self.uuid()
        )
        .fetch_one(pool)
        .await
        {
            Ok(result) => result.exists.unwrap_or(false),
            Err(e) => return Err(e),
        };

        Ok(tracked)
    }
}

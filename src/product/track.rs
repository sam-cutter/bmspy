use super::Product;
use sqlx::{Error, PgPool};

impl Product {
    async fn insert_into_products_table(&self, pool: &PgPool) -> Result<u64, sqlx::Error> {
        match sqlx::query!(
            r#"
            INSERT INTO products (uuid, title)
            VALUES ($1, $2)
            ON CONFLICT (uuid) DO NOTHING
            "#,
            self.uuid(),
            self.title()
        )
        .execute(pool)
        .await
        {
            Ok(result) => return Ok(result.rows_affected()),
            Err(e) => return Err(e),
        }
    }

    pub async fn track(&self, pool: &PgPool) -> Result<u64, Error> {
        let tracked = match self.tracked(pool).await {
            Ok(tracked) => tracked,
            Err(e) => return Err(e),
        };

        if tracked {
            return Ok(0);
        }

        match self.insert_into_products_table(pool).await {
            Ok(rows_affected) => Ok(rows_affected),
            Err(e) => Err(e),
        }
    }
}

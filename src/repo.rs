use anyhow::Result;
use chrono::NaiveDate;
use sqlx::SqlitePool;

use crate::transfer::Transfer;

pub struct TransferRepo {
    pub pool: SqlitePool
}

impl TransferRepo {
    pub async fn create(&self, value: &Transfer) -> Result<Transfer> {
        let rows = sqlx::query_as(
            r#"
        INSERT INTO transfer (date, label, value, category)
        VALUES (?, ?, ROUND(?, 2), (
            SELECT category
            FROM category_pattern
            WHERE ? LIKE pattern || '%'
            ORDER BY length(pattern) DESC
            LIMIT 1
        ))
        ON CONFLICT(date, label, value) DO UPDATE SET
            category = (
                SELECT category
                FROM category_pattern
                WHERE excluded.label LIKE pattern || '%'
                ORDER BY length(pattern) DESC
                LIMIT 1
            )
        RETURNING *;
        "#,
        )
        .bind(value.date)
        .bind(value.label.clone())
        .bind(value.amount)
        .bind(value.label.clone())
        .fetch_one(&self.pool)
        .await?;
    
        Ok(rows)
    }
    
    pub async fn get_amount_by_category(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<(String, f32)>> {
        let year_month = &date.to_string()[..7];
        let rows = sqlx::query_as(
            r#"
            SELECT
                CASE
                    WHEN t.category IS NULL THEN '__aucune__'
                    ELSE c.label
                END AS cat_label,
                SUM(value) AS amount
            FROM transfer t
            LEFT JOIN category c ON c.code = t.category
            WHERE
                SUBSTR(date, 1, 7) = ?
            GROUP BY
                CASE
                    WHEN t.category IS NULL THEN '__aucune__'
                    ELSE c.label
                END;
        "#,
        )
        .bind(year_month)
        .fetch_all(&self.pool)
        .await?;
    
        Ok(rows)
    }
    
    pub async fn get_total_amount(&self, date: NaiveDate) -> Result<(String, f32)> {
        let year_month = &date.to_string()[..7];
        let rows = sqlx::query_as(
            r#"
        SELECT
    'TOTAL' AS category,
    sum(value) AS amount
    FROM transfer t
    WHERE
        substr (date,1,7) = ?
        "#,
        )
        .bind(year_month)
        .fetch_one(&self.pool)
        .await?;
    
        Ok(rows)
    }
    
}

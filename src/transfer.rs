use anyhow::Result;
use chrono::NaiveDate;
use sqlx::{prelude::FromRow, SqlitePool};

use crate::category::SALARY_CAT;

#[derive(Clone, Debug, Default, FromRow)]
pub struct Transfer {
    pub date: NaiveDate,
    pub label: String,
    pub _category: Option<String>,
    pub amount: f32,
}

impl Transfer {
    pub fn from_str(s: &str, default_date: NaiveDate) -> Result<Self> {
        let parts: Vec<&str> = s.split('\t').collect();
        let label = parts[1].replace("\"", "").to_string();
        let date = parse_date(parts[0], default_date);
        let value: f32 = parts[2].replace(",", ".").parse()?;

        let result = Transfer {
            date,
            label,
            amount: value,
            ..Default::default()
        };
        Ok(result)
    }
}

fn parse_date(value: &str, default_date: NaiveDate) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%d/%m/%Y").unwrap_or(default_date)
}

pub async fn create(value: &Transfer, pool: &SqlitePool) -> Result<Transfer> {
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
    .fetch_one(pool)
    .await?;

    Ok(rows)
}

pub async fn get_amount_by_category(
    date: NaiveDate,
    pool: &SqlitePool,
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
    .bind(SALARY_CAT)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_total_amount(date: NaiveDate, pool: &SqlitePool) -> Result<(String, f32)> {
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
    .fetch_one(pool)
    .await?;

    Ok(rows)
}

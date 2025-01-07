use anyhow::{anyhow, Context, Result};
use chrono::{Months, NaiveDate};
use sqlx::{prelude::FromRow, SqlitePool};

use crate::category::SALARY_CAT;

//const SALARY_LABEL: &str = "VIREMENT DE ARENC LOGISTIQUE";

#[derive(Debug, Default, FromRow)]
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
            AND (t.category != ? OR t.category IS NULL)
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
    AND (category != ? OR category IS NULL)
    "#,
    )
    .bind(year_month)
    .bind(SALARY_CAT)
    .fetch_one(pool)
    .await?;

    Ok(rows)
}

pub async fn find_salary_on_date(date: NaiveDate, pool: &SqlitePool) -> Result<(String, f32)> {
    let year_month = &date.to_string()[..7];
    let rows: (String, f32) = sqlx::query_as(
        r#"
        SELECT
            c.label AS category,
            value
        FROM transfer t
        LEFT JOIN category c ON c.code = t.category
        WHERE
            substr(date,1,7) = ?
            AND t.category = ?
        "#,
    )
    .bind(year_month)
    .bind(SALARY_CAT)
    .fetch_one(pool)
    .await?;

    Ok(rows)
}
///tente de récuperer le salaire du mois précédent.
///sinon celui du mois indiqué par date
pub async fn get_salary_for_date(date: NaiveDate, pool: &SqlitePool) -> Result<(String, f32)> {
    let mut salary = 0.0;
    let last_month = date
        .checked_sub_months(Months::new(1))
        .ok_or_else(|| anyhow::anyhow!("calcul mois précédent"))
        .context("calcul mois précédent")?;
    dbg!(last_month);
    if let Ok((_, amount)) = find_salary_on_date(last_month, &pool).await {
        salary = amount;
    } else if let Ok((_, amount)) = find_salary_on_date(date, &pool).await {
        salary = amount;
    } else {
        let _: Result<(), anyhow::Error> = Err(anyhow!("Aucun salaire"));
    }

    Ok((SALARY_CAT.to_string(), salary))
}

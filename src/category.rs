use sqlx::prelude::FromRow;

pub const SALARY_CAT: &str = "salary";

#[derive(Debug, FromRow)]
pub struct Category {
    pub _code: String,
}

#[derive(Debug, FromRow)]
pub struct CategoryPattern {
    pub _pattern: String,
    pub _category_code: String,
}

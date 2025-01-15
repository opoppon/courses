use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Category {
    pub _code: String,
}

#[derive(Debug, FromRow)]
pub struct CategoryPattern {
    pub _pattern: String,
    pub _category_code: String,
}

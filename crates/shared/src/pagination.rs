use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page.min(100) as i64
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
    pub total_pages: u32,
}

impl PaginationMeta {
    pub fn new(page: u32, per_page: u32, total: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as u32;
        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }
}

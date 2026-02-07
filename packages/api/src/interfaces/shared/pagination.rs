pub struct PaginationParams;

const DEFAULT_PER_PAGE: i64 = 20;
const MAX_PER_PAGE: i64 = 100;

impl PaginationParams {
    pub fn resolve(page: Option<i64>, per_page: Option<i64>) -> (i64, i64) {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, MAX_PER_PAGE);
        (page, per_page)
    }
}

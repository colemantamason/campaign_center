use crate::define_enum;

// used by redis for content cache expiry (1 day in seconds)
pub const ARTICLE_CACHE_EXPIRY_SECONDS: u64 = 86400;

define_enum! {
    pub enum ArticleType {
        Blog => ("blog", "Blog"),
        Support => ("support", "Support"),
    }
}

define_enum! {
    pub enum ArticleStatus {
        Draft => ("draft", "Draft"),
        Published => ("published", "Published"),
        Archived => ("archived", "Archived"),
    }
}

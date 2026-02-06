use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

// used by redis for content cache expiry (1 day in seconds)
pub const ARTICLE_CACHE_EXPIRY_SECONDS: u64 = 86400;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ArticleType {
    Blog,
    Support,
}

impl ArticleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArticleType::Blog => "blog",
            ArticleType::Support => "support",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "blog" => Some(ArticleType::Blog),
            "support" => Some(ArticleType::Support),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ArticleType::Blog => "Blog",
            ArticleType::Support => "Support",
        }
    }
}

impl Display for ArticleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ArticleStatus {
    Draft,
    Published,
    Archived,
}

impl ArticleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArticleStatus::Draft => "draft",
            ArticleStatus::Published => "published",
            ArticleStatus::Archived => "archived",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "draft" => Some(ArticleStatus::Draft),
            "published" => Some(ArticleStatus::Published),
            "archived" => Some(ArticleStatus::Archived),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ArticleStatus::Draft => "Draft",
            ArticleStatus::Published => "Published",
            ArticleStatus::Archived => "Archived",
        }
    }
}

impl Display for ArticleStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

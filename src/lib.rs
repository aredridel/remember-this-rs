//! A simple tool designed to allow our code to remember results on disk.

mod cache;
mod manager;
mod result;

pub use cache::Cache;
pub use manager::{CacheManager, CacheManagerOptions, CacheOptions};
pub use result::Error;

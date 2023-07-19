pub mod fetcher_models;
pub mod fetchers;
mod utils;

pub use fetchers::client_categories::*;
pub use fetchers::client_category_page_count::*;
pub use fetchers::client_products_at_page::*;
pub use fetchers::client_single_product::*;

pub use reqwest;
pub use reqwest::Client;

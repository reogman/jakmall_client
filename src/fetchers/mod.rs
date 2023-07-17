pub mod client_categories;
pub mod client_category_page_count;
pub mod client_products_at_page;
pub mod client_single_product;

mod find_description;
mod find_spdt;
mod find_title;

pub(super) use find_description::*;
pub(super) use find_spdt::*;
pub(super) use find_title::*;

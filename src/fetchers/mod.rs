pub mod g_categories;
pub mod g_category_page_count;
pub mod g_products_at_page;
pub mod g_single_product;

mod find_title;
pub(self) use find_title::*;

mod find_description;
pub(self) use find_description::*;

mod find_spdt;
pub(self) use find_spdt::*;

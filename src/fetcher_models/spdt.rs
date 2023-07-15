use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SPDT {
    pub id: String,
    pub variants: Value,
    pub sku: HashMap<String, Sku>,
    pub matrix: Value,
    pub sold: i64,
    pub url: String,
    pub store: Store,
    pub rating: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sku {
    pub id: String,
    pub sku: Value,
    pub weight: i64,
    pub images: Vec<Image>,
    pub version: i64,
    pub volume: Value,
    #[serde(rename = "guide_url")]
    pub guide_url: Value,
    pub snapshot: String,
    #[serde(rename = "in_stock")]
    pub in_stock: bool,
    #[serde(rename = "is_limited_stock")]
    pub is_limited_stock: bool,
    #[serde(rename = "limited_stock")]
    pub limited_stock: Value,
    #[serde(rename = "is_coming_soon")]
    pub is_coming_soon: bool,
    #[serde(rename = "is_new")]
    pub is_new: bool,
    #[serde(rename = "is_cheapest_event")]
    pub is_cheapest_event: bool,
    #[serde(rename = "is_special_event")]
    pub is_special_event: bool,
    pub is_promo: bool,
    #[serde(rename = "price_hash")]
    pub price_hash: String,
    pub url: String,
    #[serde(rename = "can_single_checkout")]
    pub can_single_checkout: bool,
    #[serde(rename = "can_single_checkout_mobile")]
    pub can_single_checkout_mobile: bool,
    #[serde(rename = "checkout_url")]
    pub checkout_url: String,
    #[serde(rename = "single_checkout_only")]
    pub single_checkout_only: bool,
    pub price: Price,
    #[serde(rename = "bulk_prices")]
    pub bulk_prices: Vec<Value>,
    #[serde(rename = "weight_information")]
    pub weight_information: String,
    #[serde(rename = "pre_order")]
    pub pre_order: Value,
    #[serde(rename = "sku_display")]
    pub sku_display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub icon: String,
    pub thumbnail: String,
    pub detail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub list: i64,
    #[serde(rename = "list_money")]
    pub list_money: String,
    pub normal: i64,
    #[serde(rename = "normal_money")]
    pub normal_money: String,
    #[serde(rename = "final")]
    pub final_field: i64,
    #[serde(rename = "final_money")]
    pub final_money: String,
    #[serde(rename = "standard_retail_price")]
    pub standard_retail_price: Value,
    #[serde(rename = "standard_retail_price_money")]
    pub standard_retail_price_money: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub id: String,
    pub name: String,
    pub url: String,
    pub image: String,
    pub active: bool,
    pub closed: bool,
    #[serde(rename = "closed_open_at")]
    pub closed_open_at: Value,
    #[serde(rename = "closed_note")]
    pub closed_note: Value,
    pub statistics: Statistics,
    #[serde(rename = "is_official")]
    pub is_official: bool,
    pub rating: f64,
    #[serde(rename = "review_count")]
    pub review_count: i64,
    #[serde(rename = "is_partner_store")]
    pub is_partner_store: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    #[serde(rename = "order_handled")]
    pub order_handled: i64,
    #[serde(rename = "order_shipped")]
    pub order_shipped: i64,
    #[serde(rename = "order_complained")]
    pub order_complained: i64,
    #[serde(rename = "order_success")]
    pub order_success: i64,
    #[serde(rename = "order_process_time")]
    pub order_process_time: i64,
    #[serde(rename = "average_process_time")]
    pub average_process_time: f64,
    #[serde(rename = "percentage_success")]
    pub percentage_success: i64,
    #[serde(rename = "percentage_complained")]
    pub percentage_complained: i64,
    #[serde(rename = "sku_count")]
    pub sku_count: i64,
    #[serde(rename = "sku_quantity")]
    pub sku_quantity: i64,
}

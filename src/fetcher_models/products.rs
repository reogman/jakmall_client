// use serde::Deserialize;
// use serde::Serialize;
// use serde_json::Value;

// pub type Products = Vec<Product>;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Product {
//     pub name: String,
//     pub code: String,
//     pub rating: Value,
//     #[serde(rename = "review_count")]
//     pub review_count: i64,
//     pub sold: i64,
//     pub watcher: i64,
//     #[serde(rename = "is_free_shipping")]
//     pub is_free_shipping: bool,
//     pub weight: i64,
//     pub restriction: Value,
//     pub brand: Brand,
//     pub store: Store,
//     pub warehouse: Warehouse,
//     pub sku: Value,
//     pub badges: Vec<Value>,
//     pub url: String,
//     pub matrix: Value,
//     #[serde(rename = "selected_validation")]
//     pub selected_validation: i64,
//     pub selected: i64,
//     pub variants: Value,
//     #[serde(rename = "is_owner")]
//     pub is_owner: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Brand {
//     pub name: String,
//     pub image: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Store {
//     pub name: String,
//     pub image: String,
//     #[serde(rename = "is_official")]
//     pub is_official: bool,
//     pub url: String,
//     #[serde(rename = "is_opened")]
//     pub is_opened: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Warehouse {
//     pub name: String,
//     #[serde(rename = "drop_ship")]
//     pub drop_ship: bool,
//     pub affiliate: bool,
//     #[serde(rename = "is_partner_store")]
//     pub is_partner_store: bool,
//     #[serde(rename = "display_city")]
//     pub display_city: String,
//     pub city: City,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct City {
//     pub name: String,
//     pub province: Province,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Province {
//     pub name: String,
// }

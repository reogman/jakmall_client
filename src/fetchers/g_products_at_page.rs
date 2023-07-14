use std::collections::HashMap;

use crate::{
    err_http_msg, err_parse, err_parse_msg, some_or_err,
    utils::{get_last_bracket, BracketType},
};
use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;

/// ### <u>Description</u> :
/// Digunakan untuk mendapatkan data list product
/// ### <u>Arguments</u> :
/// - `page_category_url`, contoh: "https://www.jakmall.com/monitor?page=1"
///
/// <hr/>
///
/// ### <u>Examples</u> :
/// ```no_run
/// #[tokio::main]
/// async fn main() {
///     let product_list = jakmall_rs::get_products_at_page("https://www.jakmall.com/monitor?page=1").await;
/// }
/// ```
pub async fn get_products_at_page<T>(page_category_url: T) -> Result<Vec<String>>
where
    T: Into<String>,
{
    let client = Client::new();
    let url = page_category_url.into();

    let response = client
        .get(url)
        .send()
        .await
        .context(err_http_msg!("error fetch"))?
        .text()
        .await
        .context(err_parse_msg!("error convert string"))?;

    let html = Html::parse_document(&response);
    let selector = Selector::parse("script").unwrap();

    for element in html.select(&selector) {
        if element.html().contains("var result =") {
            let find = r#""products":"#;
            let content = element.html();

            let start_trim =
                some_or_err!(content.find(find), "products key not found") + find.len();
            let end_trim = get_last_bracket(
                content.get(start_trim..).unwrap_or(""),
                start_trim,
                BracketType::Square,
            );

            let str_object = content
                .get(start_trim..end_trim + 1)
                .ok_or_else(|| anyhow!("object string not found"))?;

            let products = serde_json::from_str::<Vec<HashMap<&str, Value>>>(str_object).context(
                err_parse_msg!("error serialize while convert object string to json model",),
            )?;

            let products: Vec<String> = products
                .iter()
                .filter_map(|p| p.get("url"))
                .filter_map(|p| p.as_str())
                .map(|p| p.to_string())
                .collect();

            return Ok(products);
        }
    }

    err_parse!("products in page is not found")
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        let url = "http://jakmall.com/aksesoris-handphone?page=93";
        let res = super::get_products_at_page(url).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }
}

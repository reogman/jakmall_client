use anyhow::{Context, Result};
use reqwest::Client;
use scraper::Html;

use crate::{err_http_msg, err_parse_msg, fetcher_models::product::Product};

/// ### <u>Description</u> :
/// Digunakan untuk mendapatkan data single product
/// ### <u>Arguments</u> :
/// - `url`, contoh: "https://www.jakmall.com/smart-shop/taffgo-smartphone-cooling-fan-kipas-pendingin-radiator-heat-sink-h-15#2985927634130"
///
/// <hr/>
///
/// ### <u>Examples</u> :
/// ```no_run
/// #[tokio::main]
/// async fn main() {
///     let url = "https://www.jakmall.com/smart-shop/taffgo-smartphone-cooling-fan-kipas-pendingin-radiator-heat-sink-h-15#2985927634130";
///     let product_info = jakmall_rs::get_single_product(url).await;
/// }
/// ```
pub async fn get_single_product<S>(url: S) -> Result<Product>
where
    S: Into<String>,
{
    let client = Client::new();
    let res = client
        .get(url.into())
        .send()
        .await
        .context(err_http_msg!("Fail to send GET PRODUCT request"))?
        .text()
        .await
        .context(err_parse_msg!("error convert string"))?;

    let html = Html::parse_document(&res);

    let title = super::find_title(&html)?;
    let desc = super::find_description(&html)?;

    Ok(Product {
        name: title,
        description: desc,
    })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        let url = "https://www.jakmall.com/smart-shop/taffgo-smartphone-cooling-fan-kipas-pendingin-radiator-heat-sink-h-15#2985927634130";
        let info = super::get_single_product(url).await;

        assert!(info.is_ok());
        assert!(!info.unwrap().name.is_empty());
    }
}

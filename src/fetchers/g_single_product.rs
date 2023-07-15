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

    let name = super::find_title(&html)?;
    let description = super::find_description(&html)?;
    let _spdt = super::find_spdt(&html)?;

    Ok(Product { name, description })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        // let url = "https://www.jakmall.com/gudanggrosir/iglove-sarung-tangan-touch-screen-untuk-smartphones-tablet#8011694424637";
        let url = "https://www.jakmall.com/kitchen-depot/one-two-cups-teko-kopi-french-press-coffee-maker-pot-kg73i#9714685366995";
        let info = super::get_single_product(url).await;

        assert!(info.is_ok());
        assert!(!info.unwrap().name.is_empty());
    }

    #[tokio::test]
    async fn sequence_test() {
        let urls = [
            "https://www.jakmall.com/gudanggrosir/iglove-sarung-tangan-touch-screen-untuk-smartphones-tablet#8011694424637",
            "https://www.jakmall.com/kitchen-depot/one-two-cups-teko-kopi-french-press-coffee-maker-pot-kg73i#9714685366995",
            "https://www.jakmall.com/indo-audio/kebidu-usb-dongle-hifi-audio-bluetooth-transmitter-receiver-kn320#6981341576545"
        ];

        for url in urls {
            let info = super::get_single_product(url).await;

            assert!(info.is_ok(), "{url} {:?}", info);
            assert!(!info.unwrap().name.is_empty());
        }
    }
}

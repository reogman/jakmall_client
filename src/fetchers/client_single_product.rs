use anyhow::{Context, Result};
use scraper::Html;

use crate::{err_http_msg, err_parse_msg, fetcher_models::product::Product};

#[derive(Debug, Clone, Default)]
pub struct ClientSingleProduct {
    client: reqwest::Client,
}

impl ClientSingleProduct {
    pub fn new() -> Self {
        ClientSingleProduct::default()
    }

    pub fn new_with_client(client: reqwest::Client) -> Self {
        ClientSingleProduct { client }
    }

    /// ### <u>Description</u> :
    /// Digunakan untuk mendapatkan data single product
    /// ### <u>Arguments</u> :
    /// - `url`, contoh: "https://www.jakmall.com/smart-shop/taffgo-smartphone-cooling-fan-kipas-pendingin-radiator-heat-sink-h-15#2985927634130"
    ///
    /// <hr/>
    ///
    /// ### <u>Examples</u> :
    /// ```no_run
    /// use jakmall_client::ClientSingleProduct;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let url = "https://www.jakmall.com/kitchen-depot/one-two-cups-teko-kopi-french-press-coffee-maker-pot-kg73i#9714685366995";
    ///     let client = ClientSingleProduct::default();
    ///     let info = client.get_single_product(url).await;
    ///     assert!(info.is_ok());
    /// }
    /// ```
    pub async fn get_single_product<S>(&self, url: S) -> Result<Product>
    where
        S: Into<String>,
    {
        let res = self
            .client
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
}

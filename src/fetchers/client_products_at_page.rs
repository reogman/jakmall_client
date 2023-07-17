use anyhow::{Context, Result};
use boa_engine::{Context as BContext, Source};
use scraper::{Html, Selector};

use crate::{err_http_msg, err_parse, err_parse_msg};

#[derive(Debug, Default, Clone)]
pub struct ClientProductsAtPage {
    client: reqwest::Client,
}

impl ClientProductsAtPage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// ### <u>Description</u> :
    /// Digunakan untuk mendapatkan data list product
    /// ### <u>Arguments</u> :
    /// - `page_category_url`, contoh: "https://www.jakmall.com/monitor?page=1"
    ///
    /// <hr/>
    ///
    /// ### <u>Examples</u> :
    /// ```no_run
    /// use jakmall_client::ClientProductsAtPage;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let url = "http://jakmall.com/aksesoris-handphone?page=81";
    ///     let client = ClientProductsAtPage::default();
    ///
    ///     let p = client.get_products_at_page(url).await;
    ///     assert!(p.is_ok());
    /// }
    /// ```
    pub async fn get_products_at_page<T>(&self, page_category_url: T) -> Result<Vec<String>>
    where
        T: Into<String>,
    {
        let url = page_category_url.into();

        let response = self
            .client
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
            if element.inner_html().contains("var result =") {
                let mut content = element.inner_html();
                content.push_str("result.products.map(p => p.url).join('%comma%')");

                let mut ctx = BContext::default();

                let js_eval = ctx
                    .eval(Source::from_bytes(&content))
                    .or_else(|e| err_parse!(e.to_string()))?;

                let urls_string = js_eval
                    .to_string(&mut ctx)
                    .or_else(|e| err_parse!(e.to_string()))?
                    .to_std_string()
                    .or_else(|e| err_parse!(e.to_string()))?;

                let urls = urls_string
                    .split("%comma%")
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>();

                return Ok(urls);
            }
        }

        err_parse!("products in page is not found")
    }
}

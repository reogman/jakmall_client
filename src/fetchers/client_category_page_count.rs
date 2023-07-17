use anyhow::{Context, Result};
use boa_engine::Source;
use scraper::{Html, Selector};

use crate::{err_http_msg, err_parse, err_parse_msg};

#[derive(Debug, Default, Clone)]
pub struct ClientCategoryPageCount {
    client: reqwest::Client,
}

impl ClientCategoryPageCount {
    pub fn new() -> Self {
        ClientCategoryPageCount {
            client: reqwest::Client::new(),
        }
    }

    pub fn new_with_client(client: reqwest::Client) -> Self {
        ClientCategoryPageCount { client }
    }

    /// Digunakan untuk mendapatkan jumlah total halaman pada kategori tertentu
    ///
    /// ## Examples
    /// ```
    /// use anyhow::{Result, Context};
    /// use jakmall_client::ClientCategoryPageCount;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cat_name = "baterai";
    ///     let client = ClientCategoryPageCount::default();
    ///     let max = client.get_category_page_count(cat_name).await;

    ///     assert!(max.is_ok());
    /// }
    /// ```
    pub async fn get_category_page_count(&self, category_name: &str) -> Result<usize> {
        let url = format!("https://www.jakmall.com/{}", category_name);

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
                content.push_str("result.pagination.total;");

                let mut ctx = boa_engine::Context::default();
                let js_eval = ctx
                    .eval(Source::from_bytes(&content))
                    .or_else(|e| err_parse!(e.to_string()))?;

                let total = js_eval
                    .to_string(&mut ctx)
                    .or_else(|e| err_parse!(e.to_string()))?
                    .to_std_string()
                    .or_else(|e| err_parse!(e.to_string()))?
                    .parse::<f64>()?;

                content.push_str("\nresult.pagination.last_item;");

                let js_eval = ctx
                    .eval(Source::from_bytes(&content))
                    .or_else(|e| err_parse!(e.to_string()))?;

                let last_item = js_eval
                    .to_string(&mut ctx)
                    .or_else(|e| err_parse!(e.to_string()))?
                    .to_std_string()
                    .or_else(|e| err_parse!(e.to_string()))?
                    .parse::<f64>()?;

                let max_page = total / last_item;
                return Ok(max_page.ceil() as usize);
            }
        }

        err_parse!("max page not found")
    }
}

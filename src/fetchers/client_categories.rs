use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};

use crate::{err_http_msg, err_parse_msg, fetcher_models::category::JakmallCategories};

#[derive(Debug, Clone)]
pub struct ClientCategories {
    client: reqwest::Client,
    url: String,
}

impl ClientCategories {
    pub fn new() -> Result<Self> {
        let url = String::from("https://www.jakmall.com/product-category-navigation");
        let default_headers: Vec<(&str, HeaderValue)> = vec![
            (
                "X-Requested-With",
                HeaderValue::from_static("XMLHttpRequest"),
            ),
            ("Host", HeaderValue::from_static("www.jakmall.com")),
        ];

        let mut headers = HeaderMap::new();

        for (name, value) in default_headers {
            headers.append(name, value);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context(err_http_msg!("Fail to create GET CATEGORIES client"))?;

        Ok(ClientCategories { client, url })
    }

    pub fn new_with_client(client: reqwest::Client) -> Self {
        let url = String::from("https://www.jakmall.com/product-category-navigation");
        ClientCategories { client, url }
    }

    /// Digunakan untuk mendapatkan data kategori json dari `jakmall`
    pub async fn get_categories(&self) -> Result<JakmallCategories> {
        let response = self
            .client
            .get(&self.url)
            .send()
            .await
            .context(err_http_msg!("Fail to send GET CATEGORIES request"))?
            .json::<JakmallCategories>()
            .await
            .context(err_parse_msg!(
                "Fail to retrieve data GET CATEGORIES request"
            ))?;

        Ok(response)
    }
}

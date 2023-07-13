use crate::{err_http_msg, err_parse_msg};
use anyhow::{Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};

pub type JakmallCategories = Vec<Parent>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub name: String,
    pub url: String,
    pub icon: String,
    pub color: Option<String>,
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub name: String,
    pub url: String,
    pub children: Vec<Children2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    pub name: String,
    pub url: String,
}

/// Digunakan untuk mendapatkan data kategori json dari `jakmall`
pub async fn get_categories() -> Result<JakmallCategories> {
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

    let target_url = "https://www.jakmall.com/product-category-navigation";
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .context(err_http_msg!("Fail to create GET CATEGORIES client"))?;

    let response = client
        .get(target_url)
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        let cats = super::get_categories().await;

        assert!(cats.is_ok());
    }

    #[tokio::test]
    async fn check_response_len() {
        let cats = super::get_categories().await;

        assert!(cats.is_ok());
        let cats = cats.unwrap();

        assert!(cats.len() > 10);
    }
}

use crate::{err_http_msg, err_parse, err_parse_msg, some_or_err, utils::get_last_bracket};
use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;

/// Digunakan untuk mendapatkan jumlah total halaman pada kategori tertentu
///
/// ## Examples
/// ```
/// use anyhow::{Result, Context};
///
/// #[tokio::main]
/// async fn main() {
///     let max_page = jakmall_rs::get_category_page_count("aksesoris-handphone")
///         .await
///         .context("Gagal mendapatkan halaman maksimal")
///         .unwrap();
/// }
/// ```
pub async fn get_category_page_count(category_name: &str) -> Result<usize> {
    let client = Client::new();
    let url = format!("https://www.jakmall.com/{}", category_name);

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
            let find = r#""pagination":"#;
            let content = element.html();

            let start_trim =
                some_or_err!(content.find(find), "pagination key not found") + find.len();
            let end_trim = get_last_bracket(content.get(start_trim..).unwrap_or(""), start_trim);

            let str_object = some_or_err!(
                content.get(start_trim..end_trim + 1),
                "object string not found"
            );

            let json = serde_json::from_str::<Value>(str_object).context(err_parse_msg!(
                "error while convert object string to json values",
            ))?;

            let json = some_or_err!(json.as_object(), "error while treat json as object");
            let total = some_or_err!(json.get("total"), "error while find total key");
            let last_count = some_or_err!(json.get("last_item"), "error while find last_item key");

            let r_total = some_or_err!(total.as_f64(), "error while treat total as number");
            let r_last_count =
                some_or_err!(last_count.as_f64(), "error while treat last_item as number");

            let max_page = r_total / r_last_count;
            return Ok(max_page.ceil() as usize);
        }
    }

    err_parse!("max page not found")
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        let max_page = super::get_category_page_count("aksesoris-handphone").await;
        assert!(max_page.is_ok());
    }

    #[tokio::test]
    async fn check_max_page() {
        let couples = [
            ("aksesoris-handphone", 137),
            ("mouse", 9),
            ("kamera-instan", 1),
        ];

        for (cat_name, expected) in couples {
            let max_page = super::get_category_page_count(cat_name).await;
            assert!(max_page.is_ok());

            assert_eq!(max_page.unwrap(), expected);
        }
    }

    #[tokio::test]
    async fn must_error() {
        let max_page = super::get_category_page_count("handphone-tablet").await;
        assert!(max_page.is_err());
        assert!(max_page.err().is_some());
    }
}

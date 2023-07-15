use crate::{err_http_msg, err_parse, err_parse_msg};
use anyhow::{Context, Result};
use boa_engine::Source;
use reqwest::Client;
use scraper::{Html, Selector};

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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        let max_page = super::get_category_page_count("baterai").await;
        println!("{:?}", max_page);
        assert!(max_page.is_ok());
    }

    #[tokio::test]
    async fn check_max_page() {
        let couples = [
            ("aksesoris-handphone", 0),
            ("mouse", 0),
            ("sarung-tangan-pria", 0),
            ("adaptor-charger", 0),
            ("screen-guard-tablet", 0),
        ];

        for (cat_name, expected) in couples {
            let max_page = super::get_category_page_count(cat_name).await;
            assert!(max_page.is_ok());

            assert!(max_page.unwrap() >= expected);
        }
    }

    #[tokio::test]
    async fn must_error() {
        let max_page = super::get_category_page_count("handphone-tablet").await;
        assert!(max_page.is_err());
        assert!(max_page.err().is_some());
    }
}

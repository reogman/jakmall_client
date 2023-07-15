use crate::{err_http_msg, err_parse, err_parse_msg};
use anyhow::{Context, Result};
use boa_engine::{Context as BContext, Source};
use reqwest::Client;
use scraper::{Html, Selector};

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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn initial_test() {
        let url = "http://jakmall.com/aksesoris-handphone?page=81";
        let res = super::get_products_at_page(url).await;
        // let _ = super::get_products_at_page(url).await;

        // println!("{:?}", res);
        assert!(res.is_ok());
    }
}

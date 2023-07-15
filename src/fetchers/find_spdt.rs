use anyhow::{Context, Result};
use boa_engine::{Context as BContext, Source};
use scraper::{Html, Selector};

use crate::{err_parse, fetcher_models::spdt::SPDT};

pub fn find_spdt(html: &Html) -> Result<SPDT> {
    let selector = Selector::parse("script").unwrap();

    for element in html.select(&selector) {
        let must_contains = "var spdt =";
        let mut content = element.inner_html();
        content.push_str("JSON.stringify(spdt)");

        if content.contains(must_contains) {
            let mut ctx = BContext::default();

            let js_eval = ctx
                .eval(Source::from_bytes(&content))
                .or_else(|e| err_parse!("{}", e.to_string()))?;

            let spdt_string = js_eval
                .to_string(&mut ctx)
                .or_else(|e| err_parse!("{}", e.to_string()))?
                .to_std_string()
                .or_else(|e| err_parse!("{}", e.to_string()))?;

            let spdt =
                serde_json::from_str::<SPDT>(&spdt_string).context("convert str to SPDT failed")?;

            return Ok(spdt);
        }
    }

    err_parse!("spdt not found")
}

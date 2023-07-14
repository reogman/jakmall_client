use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};

use crate::some_or_err;

pub fn find_description(html: &Html) -> Result<String> {
    let re = Regex::new(r"(\n\s+)").unwrap();
    let re2 = Regex::new(r"(\t+)").unwrap();
    let desc_selector = Selector::parse("div.dp__info__wrapper").unwrap();
    let desc = some_or_err!(
        html.select(&desc_selector).next(),
        "description element not found"
    )
    .text()
    .map(|t| {
        if re.is_match(t) {
            return "\n";
        }
        t
    })
    .filter(|t| {
        if re2.is_match(t) {
            return false;
        }
        true
    })
    .collect::<Vec<_>>()
    .join("");

    Ok(desc)
}

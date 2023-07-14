use anyhow::Result;
use scraper::{Html, Selector};

use crate::some_or_err;

pub fn find_title(html: &Html) -> Result<String> {
    let title_selector = Selector::parse("div.crumb__link.crumb__link--last > span").unwrap();
    let title = some_or_err!(
        html.select(&title_selector).next(),
        "title element not found"
    )
    .inner_html();

    Ok(title)
}

use jakmall_client::ClientCategories;
use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::test]
async fn get_cats_default() {
    let client = ClientCategories::new();

    assert!(client.is_ok());

    if let Ok(client) = client {
        let cats = client.get_categories().await;

        assert!(cats.is_ok());

        if let Ok(cats) = cats {
            assert!(cats.len() > 10)
        }
    }
}

#[tokio::test]
async fn get_cats_custom_err() {
    let custom_client = reqwest::Client::builder();
    let custom_client = custom_client.build().unwrap();

    let client = ClientCategories::new_with_client(custom_client);
    let cats = client.get_categories().await;

    assert!(cats.is_err())
}

#[tokio::test]
async fn get_cats_custom_ok() {
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

    let custom_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let client = ClientCategories::new_with_client(custom_client);
    let cats = client.get_categories().await;

    assert!(cats.is_ok());

    if let Ok(cats) = cats {
        assert!(cats.len() > 10)
    }
}

use jakmall_client::ClientCategoryPageCount;

#[tokio::test]
async fn get_max_page_default() {
    let cat_name = "baterai";
    let client = ClientCategoryPageCount::default();
    let max = client.get_category_page_count(cat_name).await;

    assert!(max.is_ok());
}

#[tokio::test]
async fn get_max_page_sequential() {
    let cats = [
        "aksesoris-handphone",
        "mouse",
        "sarung-tangan-pria",
        "adaptor-charger",
        "screen-guard-tablet",
    ];

    for cat in cats {
        let client = ClientCategoryPageCount::default();
        let max = client.get_category_page_count(cat).await;

        assert!(max.is_ok());
    }
}

use jakmall_client::ClientProductsAtPage;

#[tokio::test]
async fn get_product_at_page_default() {
    let url = "http://jakmall.com/aksesoris-handphone?page=81";
    let client = ClientProductsAtPage::default();

    let p = client.get_products_at_page(url).await;
    assert!(p.is_ok());
}

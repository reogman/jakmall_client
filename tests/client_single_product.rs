use jakmall_client::ClientSingleProduct;

#[tokio::test]
async fn get_single_product_default() {
    let url = "https://www.jakmall.com/kitchen-depot/one-two-cups-teko-kopi-french-press-coffee-maker-pot-kg73i#9714685366995";
    let client = ClientSingleProduct::default();

    let info = client.get_single_product(url).await;
    assert!(info.is_ok());
}

#[tokio::test]
async fn get_single_product_sequential() {
    let urls = [
        "https://www.jakmall.com/gudanggrosir/iglove-sarung-tangan-touch-screen-untuk-smartphones-tablet#8011694424637",
        "https://www.jakmall.com/kitchen-depot/one-two-cups-teko-kopi-french-press-coffee-maker-pot-kg73i#9714685366995",
        "https://www.jakmall.com/indo-audio/kebidu-usb-dongle-hifi-audio-bluetooth-transmitter-receiver-kn320#6981341576545",
    ];

    for url in urls {
        let client = ClientSingleProduct::default();
        let info = client.get_single_product(url).await;

        assert!(info.is_ok(), "{url} {:?}", info);
        assert!(!info.unwrap().name.is_empty());
    }
}

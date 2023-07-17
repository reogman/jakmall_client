# jakmall_client
![test](https://github.com/causal-agent/scraper/actions/workflows/test.yml/badge.svg)

## Deskripsi
Pustaka `jakmal_client` untuk mengambil data dari jakmall. Seperti: data list kategori, data maksimal halaman pada kategori tertentu, data info produk, dll.   

## Instalasi
```shell
cargo add --git "https://github.com/reogman/jakmall_client.git"`
```

<p>atau</p>

```toml
[dependencies]
jakmal_client = { git = "https://github.com/reogman/jakmall_client.git" }
```

## Penggunaan
Beberapa fungsi yang telah tercover dan telah ditest dengan sempurna antara lain:

-   `ClientCategories` struct berfungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.

    ```rust
    // Default client
    use jakmall_client::ClientCategories;

    #[tokio::main]
    async fn main() {
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
    ```

-   `ClientCategoryPageCount` struct berfungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.

    ```rust
    // Default client
    use jakmall_client::ClientCategoryPageCount;

    #[tokio::main]
    async fn main() {
        let cat_name = "baterai";
        let client = ClientCategoryPageCount::default();
        let max = client.get_category_page_count(cat_name).await;

        assert!(max.is_ok());
    }
    ```

-   `ClientProductsAtPage` struct berfungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.

    ```rust
    // Default client
    use jakmall_client::ClientProductsAtPage;

    #[tokio::main]
    async fn main() {
        let url = "http://jakmall.com/aksesoris-handphone?page=81";
        let client = ClientProductsAtPage::default();

        let p = client.get_products_at_page(url).await;
        assert!(p.is_ok());
    }
    ```

-   `ClientSingleProduct` struct berfungsi untuk mendapatkan info detail product.

    ```rust
    // Default client
    use jakmall_client::ClientSingleProduct;

    #[tokio::main]
    async fn main() {
        let url = "https://www.jakmall.com/kitchen-depot/one-two-cups-teko-kopi-french-press-coffee-maker-pot-kg73i#9714685366995";
        let client = ClientSingleProduct::default();

        let info = client.get_single_product(url).await;
        assert!(info.is_ok());
    }
    ```

## Coverage / TODO

-   [x] Fungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.
-   [x] Fungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.
-   [x] Fungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.
-   [x] Fungsi untuk mendapatkan data detail pada halaman utama single produk.

## Creator

| Avatar                                                                                                    | Username  |
| --------------------------------------------------------------------------------------------------------- | --------- |
| <img width="40" style="border-radius: 40px;" src="https://avatars.githubusercontent.com/u/88225964?v=4"/> | zahrulsch |

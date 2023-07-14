## jakmall_rs

![test](https://github.com/causal-agent/scraper/actions/workflows/test.yml/badge.svg)

Library `jakmal_rs` jakmall data ekstraktor, beberapa fungsi yang telah tercover dan telah ditest dengan sempurna antara lain:

-   `get_categories` berfungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.
-   `get_category_page_count` berfungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.
-   `get_products_at_page` berfungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.
-   `get_single_product` berfungsi untuk mendapatkan info detail product.

## Instalasi

```shell
cargo add --git "https://github.com/reogman/jakmall_rs.git"`
```

<i>atau</i>

```toml
[dependencies]
jakmal_rs = { git = "https://github.com/reogman/jakmall_rs.git" }
```

## Penggunaan

-   Mendapatkan daftar kategori.

```rs
use jakmall::{get_categories, JakmallCategories};

#[tokio::main]
async fn main() -> anyhow::Result<JakmallCategories> {
    let cats = get_categories().await?;
    Ok(cats)
}
```

-   Mendapatkan batas halaman / max page pada kategori `aksesoris-handphone`.

```rs
use anyhow::{Result, Context};

#[tokio::main]
async fn main() -> Result<()> {
    let max_page = jakmall_rs::get_category_page_count("aksesoris-handphone")
        .await
        .context("Gagal mendapatkan halaman maksimal")?;

    Ok(())
}
```

-   Mendapatkan daftar produk pada kategori `aksesoris-handphone` halaman ke `1`.

```rust
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let product_list = jakmall_rs::get_products_at_page(
            "https://www.jakmall.com/aksesoris-handphone?page=1"
        )
        .await
        .context("Gagal mendapatkan data")?;

    Ok(())
}
```

-   Mendapatkan info detail produk.

```rs
#[tokio::main]
async fn main() {
    let url = "https://www.jakmall.com/smart-shop/taffgo-smartphone-cooling-fan-kipas-pendingin-radiator-heat-sink-h-15#2985927634130";
    let product_info = jakmall_rs::get_single_product(url).await;
}
```

## Coverage / TODO

-   [x] fungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.
-   [x] fungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.
-   [x] fungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.
-   [x] fungsi untuk mendapatkan data detail pada halaman utama single produk.

## Creator

|                                                                                                           | Username  |
| --------------------------------------------------------------------------------------------------------- | --------- |
| <img width="40" style="border-radius: 40px;" src="https://avatars.githubusercontent.com/u/88225964?v=4"/> | zahrulsch |

<img src="https://github-readme-stats.vercel.app/api/top-langs/?username=zahrulsch"/>

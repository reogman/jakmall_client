
## jakmall_rs

![test](https://github.com/causal-agent/scraper/actions/workflows/test.yml/badge.svg)

Library `jakmal_rs` jakmall data ekstraktor, beberapa fungsi yang telah tercover dan telah ditest dengan sempurna antara lain:

- `get_categories` berfungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.
- `get_category_page_count` berfungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.
- `get_products_at_page` berfungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.

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

- Mendapatkan daftar kategori.
```rs
use jakmall::{get_categories, JakmallCategories};

#[tokio::main]
async fn main() -> anyhow::Result<JakmallCategories> {
    let cats = get_categories().await?;
    Ok(cats)
}
```

- Mendapatkan batas halaman / max page pada kategori `aksesoris-handphone`
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

- Mendapatkan daftar produk pada kategori `aksesoris-handphone` halaman ke `1`
```rust
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let product_list = jakmall_rs::get_products_at_page("https://www.jakmall.com/aksesoris-handphone?page=1")
        .await
        .context("Gagal mendapatkan data")?;

    Ok(())
}
```

## Coverage / TODO

- [x] fungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.
- [x] fungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.
- [x] fungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.
- [ ] fungsi untuk mendapatkan data detail pada halaman utama single produk.
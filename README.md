
## jakmall_rs

![test](https://github.com/causal-agent/scraper/actions/workflows/test.yml/badge.svg)

Library `jakmal_rs` jakmall data ekstraktor, beberapa fungsi yang telah tercover dan telah ditest dengan sempurna antara lain:

- `get_categories` berfungsi untuk mendapatkan daftar kategori yang tersedia di jakmall.
- `get_category_page_count` berfungsi untuk mendapatkan jumlah maksimal halaman yang tersedia pada masing-masing kategori.
- `get_products_at_page` berfungsi untuk mendapatkan list url product pada halaman kategori di page tertentu.

## Instalasi

`cargo add --git "https://github.com/reogman/jakmall_rs.git"` <br>

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
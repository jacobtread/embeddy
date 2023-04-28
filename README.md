# Embeddy 

![License](https://img.shields.io/github/license/jacobtread/embeddy?style=for-the-badge)
![Cargo Version](https://img.shields.io/crates/v/embeddy?style=for-the-badge)
![Cargo Downloads](https://img.shields.io/crates/d/embeddy?style=for-the-badge)

> Minimal opinionated resource embedding 

Files are loaded relative to the Cargo.toml file

## Example Usage

```rust
use embeddy::Embedded;

/// Contains all the files stored in the "test" folder 
/// use the [`Embedded::get`] method to access the folders
#[derive(Embedded)]
#[folder = "test"]
struct MyResources;

fn main() {
    let file: Option<&'static [u8]> = MyResources::get("test.txt");
    let nested_file: Option<&'static [u8]> = MyResources::get("sub/test.txt");
}
```
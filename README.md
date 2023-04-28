# Embeddy 

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
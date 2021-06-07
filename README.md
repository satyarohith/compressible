# compressible

Check if a content type is compressible using compression algorithms like gzip,
brotli, deflate, etc.

## Usage

`Cargo.toml`

```toml
[dependencies]
compressible = "0.1.0"
```

`main.rs`

```rs
use compression::is_compressible;

assert_eq!(is_compressible("text/plain".to_string()), true);
assert_eq!(is_compressible("image/jpeg".to_string()), false);
```
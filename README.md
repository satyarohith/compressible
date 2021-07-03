# compressible

Check if a content type is compressible using compression algorithms like gzip,
brotli, deflate, etc.

## Usage

`Cargo.toml`

```toml
[dependencies]
compressible = "0.2.0"
```

`main.rs`

```rs
use compression::is_compressible;

assert_eq!(is_compressible("text/plain"), true);
assert_eq!(is_compressible("image/jpeg"), false);
```

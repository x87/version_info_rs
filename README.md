[![crates.io](https://img.shields.io/crates/v/version_info.svg)](https://crates.io/crates/version_info)
[![docs.rs](https://docs.rs/version_info/badge.svg)](https://docs.rs/version_info/)

# version_info

Returns a file version composed of four numbers if the given file has the version information.

## Usage

Add `version_info` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
version_info = "*"
```

## Example

```rust
let (a1, a2, a3, a4) = get_file_version("mylib.dll")?;
println!("mylib.dll's version is {}.{}.{}.{}", a1, a2, a3, a4);
```

This function returns `None` if the file does not have the version information or if the version information is invalid.

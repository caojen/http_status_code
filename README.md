# http_status_code

This library provides definition (in number, `u16`) of HTTP status code, which are defined in `rfc9110`.

## Usage
First, add this in your Cargo.toml:
```toml
[dependencies]
http_status_code = { version = "1" }
```

Then, you can use it in code, like:
```rust
fn main() {
    // Note: All constants are u16
    let _a: u16 = http_status_code::OK;
    let _b: u16 = http_status_code::Forbidden;
}
```

## Reference
`RFC9110`, Session 15: [Link](https://www.rfc-editor.org/rfc/rfc9110#name-status-codes)

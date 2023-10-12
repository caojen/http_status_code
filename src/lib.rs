//! HTTP status code
//!
//! `http_status_code` provides constants for HTTP status code, which defined in [rfc9110](https://www.rfc-editor.org/rfc/rfc9110#name-status-codes).
//!
//! ## Usage
//!
//! First, add this in your Cargo.toml:
//! ```toml
//! [dependencies]
//! http_status_code = { version = "1" }
//! ```
//!
//! Then, you can use it in code, like:
//! ```rust
//! let _ = http_status_code::OK;
//! ```
//!

mod number;

pub use number::*;

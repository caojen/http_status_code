//! HTTP status code
//!
//! `rhttp_status_code` provides constants for HTTP status code, which have been defined in [rfc9110](https://www.rfc-editor.org/rfc/rfc9110#name-status-codes).
//!
//! ## Usage
//!
//! First, add this in your Cargo.toml:
//! ```toml
//! [dependencies]
//! rhttp_status_code = { version = "1" }
//! ```
//!
//! Then, you can use it in code, like:
//! ```rust
//! // Note: All constants are u16
//! let _ = http_status_code::OK;
//! let _ = http_status_code::Forbidden;
//! ```
//!

mod number;

pub use number::*;

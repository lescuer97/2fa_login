#![warn(
    unused_extern_crates,
    missing_copy_implementations,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::fallible_impl_from,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::dbg_macro
)]
#![forbid(unsafe_code)]

pub mod auth;
pub mod routes;
pub mod server_messages;
pub mod users;
pub mod utils;

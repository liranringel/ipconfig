//! Get network adapters information for windows.
//!
//!
//! # Examples
//!
//! ```rust
//! # fn foo() -> ipconfig::error::Result<()> {
//! // Print the ip addresses and dns servers of all adapters:
//! for adapter in ipconfig::get_adapters()? {
//!     println!("Ip addresses: {:#?}", adapter.ip_addresses());
//!     println!("Dns servers: {:#?}", adapter.dns_servers());
//! }
//! # Ok(())
//! # }
//! # fn main() {
//!     # foo().unwrap();
//! # }
//! ```

#![cfg(windows)]
#![doc(html_root_url = "https://docs.rs/ipconfig/0.1.3/x86_64-pc-windows-gnu/ipconfig/")]

#[macro_use]
extern crate error_chain;
extern crate winapi;
extern crate widestring;
extern crate socket2;
extern crate winreg;


pub mod error;
pub mod computer;
mod adapter;

pub use adapter::{get_adapters, Adapter};

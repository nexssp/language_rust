//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! serde = "*"
//! serde_derive = "*"
//! serde_json = "*"
//! rustc_version = "*"
//! ```

// Nexss PROGRAMMER 2.0.0 - Rust
// Default template for JSON Data
// it's uses cargo script

// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

// extern crate rustc_version;
// use rustc_version::{version, version_meta, Channel, Version};

// STDIN
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
// use ::serde_derive::Serialize;
// use ::serde_derive::Deserialize;

// extern crate rustc_version;
// use rustc_version::{version, version_meta, Channel, Version};

// use serde_json::Value;
use std::io::{self, Write};
// use std::mem;
fn main() -> io::Result<()> {
    let mut nexss_stdout = String::new();
    // let mut json = serde_json::to_string(&my_obj).expect("Couldn't serialize config");
    match io::stdin().read_line(&mut nexss_stdout) {
        Ok(_) => {
            // println!("{} bytes read", n);
            let mut nexss_stdout:serde_json::Value = serde_json::from_str(&nexss_stdout).unwrap();
            nexss_stdout["test"] = serde_json::Value::String("test".to_string());
            io::stdout().write_all(nexss_stdout.to_string().as_bytes())?;
        }
        Err(error) => println!("error: {}", error),
    }

    Ok(())
}
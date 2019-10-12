//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! serde_derive = "*"
//! serde_json = "*"
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

use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let nexssstdin = io::stdin();
    nexssstdin.lock().read_line(&mut line).unwrap();
    //line["x"] = "dddddd";
    
    
    let mut json = serde_json::to_string(&line).expect("Couldn't serialize config");
    //json["test"] = "test";
    // STDOUT
    println!("{}", json)
}
#!/usr/bin/env run-cargo-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! serde_json = "1.0.39"
//! ```

// cargo script is used to run the rust scripts.
// more here https://github.com/DanielKeep/cargo-script
extern crate serde_json;

use serde_json::*;
use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let stream_deserializer: StreamDeserializer<Value, _> = StreamDeserializer::new(stdin.bytes());

    for v in stream_deserializer {
        match v {
            Ok(value) => {
                println!("{:?}", value);
            },
            Err(_) => println!("Encountered a json parsing error, closing"),
        }
    }
}

use std::io;
use std::io::prelude::*;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();          // Reading input from STDIN
    println!("Hi, {}.", name);                          // Writing output to STDOUT
}

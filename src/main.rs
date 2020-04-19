#![allow(dead_code)]

use crate::runtime::Jvm;
use std::env;

#[macro_use]
mod macros;
mod class_parser;
mod class_path;
mod nom_utils;
mod runtime;

fn main() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let mut jvm = Jvm::new(
        "main/Main",
        Some("./jre".to_string()),
        Some("./jre/lib/rt".to_string()),
    );
    jvm.run();
}

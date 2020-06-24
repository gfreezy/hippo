#![allow(dead_code)]

use crate::runtime::Jvm;
use std::env;
use std::fs::OpenOptions;
use std::io::BufWriter;
use tracing_subscriber::EnvFilter;

#[macro_use]
mod macros;
mod class_parser;
mod class_path;
mod nom_utils;
mod runtime;

fn main() {
    env::set_var("RUST_LOG", "debug");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("hippo.log")
        .unwrap();
    let writer = BufWriter::new(file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(writer);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .json()
        .init();

    let mut jvm = Jvm::new(
        "main/Main",
        Some("./jre".to_string()),
        Some("./jre/lib/rt".to_string()),
    );
    jvm.run();
}

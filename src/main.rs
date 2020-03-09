#![allow(dead_code)]

use crate::runtime::Jvm;

#[macro_use]
mod macros;
mod class_parser;
mod class_path;
mod nom_utils;
mod runtime;

fn main() {
    tracing_subscriber::fmt::init();

    let mut jvm = Jvm::new("main/Main");
    jvm.run();
}

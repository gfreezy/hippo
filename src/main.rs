#![allow(dead_code)]

use crate::runtime::Jvm;

#[macro_use]
mod macros;
mod class_parser;
mod class_path;
mod nom_utils;
mod runtime;

fn main() {
    let mut jvm = Jvm::new("main/Main");
    jvm.run();
}

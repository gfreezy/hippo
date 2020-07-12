use hippo::debug::{dump_space, pretty_print};
use hippo::jenv::new_java_lang_string;
use hippo::jvm::Jvm;
use std::env;

fn main() {
    let _jvm = Jvm::new(Some("./jre".to_string()), Some("./jre/lib/rt".to_string()));
    let s = new_java_lang_string("hello");
    pretty_print(s);
    dump_space();
}

extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("DEFAULTS")
        .dir("default", Compression::Gzip)
        .build("default.rs")
        .unwrap();
}

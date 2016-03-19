extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .opt_level(2)
        .define("DECODER_ONLY", None)
        .flag("-std=gnu++11")
        .file("src/c/src/library/flif-interface_dec.cpp")
        .include("src/c/src/library")
        .compile("libflif_dec.a");
}

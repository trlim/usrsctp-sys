extern crate cmake;

use cmake::Config;

fn main() {
    let mut dst = Config::new("usrsctp")
                    .build();

    dst.push("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=usrsctp");
}

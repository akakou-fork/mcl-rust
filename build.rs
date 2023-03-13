use std::process::Command;

fn main() {
    Command::new("sh")
        .args(["build.sh"])
        .output()
        .expect("fail");

    println!("cargo:rustc-link-search=./build/");
    println!("cargo:rustc-link-search=/usr/lib/arm-none-eabi/newlib/thumb/v8-m.main+fp/softfp/");
}

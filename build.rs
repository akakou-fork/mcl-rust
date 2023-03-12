use std::process::Command;

fn main() {
    Command::new("sh")
        .args(["build.sh"])
        .output()
        .expect("fail");

    println!("cargo:rustc-link-search=./build/");
}

use std::panic;
use std::path::Path;
use std::process::Command;
use std::string::String;

const MCL_FOLDER: &str = "./target/thirdparty/mcl";
const MCL_REPOSITORY: &str = "https://github.com/herumi/mcl";

fn run_command(script: &str) {
    let error_msg = format!("failed to run: {}", script);
    let output = Command::new("bash")
        .args(["-c", script])
        .output()
        .expect(&error_msg);

    String::from_utf8(output.stdout.clone()).expect("error: encode command output to utf-8");
    let stderr =
        String::from_utf8(output.stderr.clone()).expect("error: encode command output to utf-8");

    if !output.status.success() {
        panic!("error: failed to running commands\n{}", stderr);
    }
}

fn main() {
    let clone_scripts = format!("git clone {} {}", MCL_REPOSITORY, MCL_FOLDER);
    let build_scripts = format!(
        "cd {} && make lib/libmcl.a lib/libmclbn384_256.a -j4 CXX=clang++",
        // "cd {} && cmake CXX=clang+.&& make -j4 ",
        MCL_FOLDER
    );

    if !Path::new(MCL_FOLDER).exists() {
        run_command(&clone_scripts);
    }

    run_command(&build_scripts);
    println!("cargo:rustc-link-search={MCL_FOLDER}/lib");
    println!("cargo:rustc-link-search=/usr/lib/gcc-cross/arm-linux-gnueabi/8/");
    // println!("cargo:rustc-link-lib=static=stdc++");
}

use std::process::Command;
use std::env::var;


fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();

    let opt = if cfg!(target_arch = "x86_64") {
        ""
    } else {
        "-DCMAKE_CXX_COMPILER=clang++"
    };

    let cmd = format!(
        "mkdir -p build && cd build && cmake ../mcl -DMCL_STATIC_LIB=ON -DMCL_STANDALONE=ON {} && make -j",
        opt
    );
    Command::new("sh")
        .args(["-c", &cmd])
        .output()
        .expect("fail");
    
    println!("cargo:rustc-link-search={}/build", manifest_dir);
    println!("cargo:rustc-link-lib=static=mclbn384_256");
    println!("cargo:rustc-link-lib=static=mcl");
}


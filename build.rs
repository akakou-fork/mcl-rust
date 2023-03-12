use std::process::Command;

fn main() {
    let url1 = "https://web.sfc.keio.ac.jp/~t19503ka/res/libmcl.a";
    let url2 = "https://web.sfc.keio.ac.jp/~t19503ka/res/libmclbn384_256.a";

    let cmd = format!(
        "mkdir -p build/ && cd build/ && wget {} {}",
        url1, url2
    );
    Command::new("sh")
        .args(["-c", &cmd])
        .output()
        .expect("fail");
    
    println!("cargo:rustc-link-search=./build/");
}

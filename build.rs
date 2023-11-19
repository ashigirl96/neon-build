use std::env;

fn main() {
    if let Ok(val) = env::var("VOICEVOX_CORE_PATH") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", val);
    } else {
        println!("VOICEVOX_CORE_PATH is not set.");
    };
}

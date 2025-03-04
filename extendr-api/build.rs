use std::env;

fn main() {
    // The R version information is needed to handle the API differences
    // between versions. `These DEP_R_R_VERSION_*` are provided by libR-sys
    // (for more details, please refer to libR-sys's `build.rs`).
    // The current approach is to add a config flag per feature. When there are
    // too many features, we might need to consider switching to per-version
    // config flags (e.g. `r410`).
    let major = env::var("DEP_R_R_VERSION_MAJOR").unwrap();
    let minor = env::var("DEP_R_R_VERSION_MINOR").unwrap();
    // let patch = env::var("DEP_R_R_VERSION_PATCH").unwrap();

    // R_NewEnv is available as of R 4.1.0
    if &*major >= "4" && &*minor >= "1" {
        println!("cargo:rustc-cfg=use_r_newenv");
    }
}

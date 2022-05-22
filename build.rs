use std::{
    env,
    path::{Path, PathBuf},
};

use cxx_build::CFG;
use glob::glob;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let inc_dir = Path::new(&manifest_dir).join("tbb").join("include");
    let srcs: Vec<PathBuf> = glob("tbb/src/tbb/*.cpp")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    CFG.exported_header_dirs = vec![&inc_dir];

    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .files(srcs)
        .include(&inc_dir)
        .define("__TBB_BUILD", None)
        .define("__TBB_NO_IMPLICIT_LINKAGE", None)
        // MSVC flags
        .flag_if_supported("/std:c++17")
        .flag_if_supported("/Zc:preprocessor")
        .flag_if_supported("/EHsc")
        // GCC/Clang flags
        .flag_if_supported("-w")
        .flag_if_supported("-std=c++17")
        .warnings(false);

    if cfg!(target_os = "macos") {
        build.define("_XOPEN_SOURCE", None);
    }

    if cfg!(target_arch = "x86_64") {
        build.define("__TBB_USE_ITT_NOTIFY", None);
    } else {
        build.define("USE_PTHREAD", None);
    }

    if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        build.flag_if_supported("-mwaitpkg");
    }

    build.compile("tbb");
}

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const NIMBLE_DIR: &str = "./nimble";

fn config_cmake() {
    Command::new("cmake")
        .current_dir(NIMBLE_DIR)
        .args(&[
            "-B",
            "build",
            "-S",
            "src",
            "-G",
            "Unix Makefiles", // TODO: choose the generator based on platform
        ])
        .status()
        .unwrap();
}

fn link_library(name: &str, dir: &Path) {
    println!("cargo:rustc-link-search={}", dir.to_str().unwrap());
    println!("cargo:rustc-link-lib={name}");
}

fn build_cmake() {
    Command::new("cmake")
        .current_dir(NIMBLE_DIR)
        .args(&["--build", "build"])
        .status()
        .unwrap();
}

fn main() {
    config_cmake();
    build_cmake();

    let targets = [
        // main library target
        ("nimble", format!("{NIMBLE_DIR}/build/lib")),
        // Dependencies
        // note that some of the directories has a "-c" suffix and that's why we need
        // both the name and path
        (
            "assent",
            format!("{NIMBLE_DIR}/build/deps/piot/assent-c/src/lib"),
        ),
        (
            "bit-array",
            format!("{NIMBLE_DIR}/build/deps/piot/bit-array/src/lib"),
        ),
        (
            "blob-stream",
            format!("{NIMBLE_DIR}/build/deps/piot/blob-stream/src/lib"),
        ),
        ("clog", format!("{NIMBLE_DIR}/build/deps/piot/clog/src/lib")),
        (
            "datagram-transport",
            format!("{NIMBLE_DIR}/build/deps/piot/datagram-transport-c/src/lib"),
        ),
        (
            "datagram-transport-local",
            format!("{NIMBLE_DIR}/build/deps/piot/datagram-transport-local/src/lib"),
        ),
        (
            "discoid",
            format!("{NIMBLE_DIR}/build/deps/piot/discoid-c/src/lib"),
        ),
        (
            "flood",
            format!("{NIMBLE_DIR}/build/deps/piot/flood-c/src/lib"),
        ),
        (
            "hazy",
            format!("{NIMBLE_DIR}/build/deps/piot/hazy-c/src/lib"),
        ),
        (
            "imprint",
            format!("{NIMBLE_DIR}/build/deps/piot/imprint/src/lib"),
        ),
        (
            "lagometer",
            format!("{NIMBLE_DIR}/build/deps/piot/lagometer-c/src/lib"),
        ),
        (
            "monotonic-time",
            format!("{NIMBLE_DIR}/build/deps/piot/monotonic-time-c/src/lib"),
        ),
        (
            "nimble-client",
            format!("{NIMBLE_DIR}/build/deps/piot/nimble-client-c/src/lib"),
        ),
        (
            "nimble-engine-client",
            format!("{NIMBLE_DIR}/build/deps/piot/nimble-engine-client/src/lib"),
        ),
        (
            "nimble-serialize",
            format!("{NIMBLE_DIR}/build/deps/piot/nimble-serialize-c/src/lib"),
        ),
        (
            "nimble-server-lib",
            format!("{NIMBLE_DIR}/build/deps/piot/nimble-server-lib/src/lib"),
        ),
        (
            "nimble-steps",
            format!("{NIMBLE_DIR}/build/deps/piot/nimble-steps-c/src/lib"),
        ),
        (
            "nimble-steps-serialize",
            format!("{NIMBLE_DIR}/build/deps/piot/nimble-steps-serialize-c/src/lib"),
        ),
        (
            "ordered-datagram",
            format!("{NIMBLE_DIR}/build/deps/piot/ordered-datagram-c/src/lib"),
        ),
        (
            "rectify",
            format!("{NIMBLE_DIR}/build/deps/piot/rectify-c/src/lib"),
        ),
        (
            "secure-random",
            format!("{NIMBLE_DIR}/build/deps/piot/secure-random-c/src/lib"),
        ),
        (
            "seer",
            format!("{NIMBLE_DIR}/build/deps/piot/seer-c/src/lib"),
        ),
        (
            "stats",
            format!("{NIMBLE_DIR}/build/deps/piot/stats-c/src/lib"),
        ),
        (
            "time-tick",
            format!("{NIMBLE_DIR}/build/deps/piot/time-tick-c/src/lib"),
        ),
        (
            "transmute",
            format!("{NIMBLE_DIR}/build/deps/piot/transmute-c/src/lib"),
        ),
        (
            "tiny-libc",
            format!("{NIMBLE_DIR}/build/deps/piot/tiny-libc/src/lib"),
        ),
    ];

    for (name, dir) in targets.iter() {
        let dir = fs::canonicalize(dir).expect("Failed to create absolute path");
        link_library(name, &dir);
    }

    // Generate bindings with bindgen
    let bindings = bindgen::Builder::default()
        .clang_args([
            //"-Fconclave-room-lib/deps/piot/clog/src/include",
            format!("-I{NIMBLE_DIR}/src/deps/piot/clog/src/include"),
            format!("-I{NIMBLE_DIR}/src/deps/piot/transmute-c/src/include"),
            format!("-I{NIMBLE_DIR}/src/deps/piot/tiny-libc/src/include"),
        ])
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .blocklist_item("utest.h") // does this work?
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
    Command::new("cmake")
            .arg("-DCMAKE_C_FLAGS=-fPIC")
            .arg("-DGLFW_BUILD_EXAMPLES=OFF")
            .arg("-DGLFW_BUILD_TESTS=OFF")
            .arg("-DGLFW_BUILD_DOCS=OFF")
            .arg(env!("CARGO_MANIFEST_DIR"))
            .current_dir(&Path::new(env!("OUT_DIR")))
            .status()
            .ok().expect("Failed to build with cmake");

    Command::new("make")
            .current_dir(&Path::new(env!("OUT_DIR")))
            .status()
            .ok().expect("Failed to build with make");

    fs::copy(&Path::new(env!("OUT_DIR")).join("src").join("libglfw3.a"),
             &Path::new(env!("OUT_DIR")).join("libglfw3.a"))
             .ok().expect("Failed to move file");

    println!("cargo:rustc-flags=-L {} -l glfw3:static", env!("OUT_DIR"));
}

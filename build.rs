use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
    let cmake_output = Command::new("cmake")
            .arg("-GUnix Makefiles")
            .arg("-DCMAKE_C_FLAGS=-fPIC")
            .arg("-DGLFW_BUILD_EXAMPLES=OFF")
            .arg("-DGLFW_BUILD_TESTS=OFF")
            .arg("-DGLFW_BUILD_DOCS=OFF")
            .arg(env!("CARGO_MANIFEST_DIR"))
            .current_dir(&Path::new(env!("OUT_DIR")))
            .output()
            .unwrap_or_else(|e| { panic!("Failed to execute cmake: {}", e) });

    if !cmake_output.status.success() {
        panic!("cmake exited with error {}:\n{}\n{}",
            cmake_output.status.code().unwrap(),
            String::from_utf8_lossy(&cmake_output.stdout),
            String::from_utf8_lossy(&cmake_output.stderr));
    }

    let make_output = Command::new("make")
            .current_dir(&Path::new(env!("OUT_DIR")))
            .output()
            .unwrap_or_else(|e| { panic!("Failed to execute make: {}", e) });

    if !make_output.status.success() {
        panic!("make exited with error {}:\n{}\n{}",
            make_output.status.code().unwrap(),
            String::from_utf8_lossy(&make_output.stdout),
            String::from_utf8_lossy(&make_output.stderr));
    }

    fs::copy(&Path::new(env!("OUT_DIR")).join("src").join("libglfw3.a"),
             &Path::new(env!("OUT_DIR")).join("libglfw3.a"))
             .ok().expect("Failed to move file");

    println!("cargo:rustc-flags=-L {} -l glfw3:static", env!("OUT_DIR"));
}

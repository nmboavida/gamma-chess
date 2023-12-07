fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").expect("Unable to get TARGET_OS");
    match os.as_str() {
        "linux" | "windows" => {
            if let Some(lib_path) = std::env::var_os("DEP_TCH_LIBTORCH_LIB") {
                println!(
                    "cargo:rustc-link-arg=-Wl,-rpath={}",
                    lib_path.to_string_lossy()
                );
            }
            // This line tells the linker to link all specified libraries regardless
            // of whether they are being used or not.
            //
            // The -Wl option passes the subsequent comma-separated arguments
            // directly to the linker.
            //
            // The --no-as-needed flag is a linker option that prevents the
            // linker from dropping any libraries that it thinks are not used.
            println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
            // This line is another directive to the linker.
            //
            // The --copy-dt-needed-entries option ensures that the dynamic
            // linking dependencies (specified in DT_NEEDED entries) of the
            // libraries that are being linked are also pulled in.
            //
            // This is particularly important in certain linking scenarios
            // where dependencies of linked libraries are not automatically included.
            println!("cargo:rustc-link-arg=-Wl,--copy-dt-needed-entries");
            // This line specifies that the torch library should be
            // linked to the final binary.
            // The -l prefix is a standard way to tell the linker to link a
            // library. Here, it specifies libtorch, which is typically
            // represented as libtorch.so on Linux or libtorch.dylib on macOS.
            // It's important to ensure that this library is available
            // in your library path, otherwise, the linker will not be able to find and link it.
            println!("cargo:rustc-link-arg=-ltorch");
        }
        "macos" => {
            // https://pyo3.rs/v0.14.2/building_and_distribution.html#macos
            pyo3_build_config::add_extension_module_link_args();
            // macOS-specific configurations
            if let Some(lib_path) = std::env::var_os("DEP_TCH_LIBTORCH_LIB") {
                println!(
                    "cargo:rustc-link-arg=-Wl,-rpath,{}",
                    lib_path.to_string_lossy()
                );
                panic!("YOOO");
            }
            // Add any macOS-specific linker arguments here, if necessary
        }
        _ => {}
    }
}

fn main() {
    #[cfg(not(feature = "enable_ffi"))]
    println!("cargo:rustc-link-search=./target/debug");
    #[cfg(not(feature = "enable_ffi"))]
    println!("cargo:rustc-link-search=C:/Users/tester/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/windows_x86_64_msvc-0.52.6/lib");
}

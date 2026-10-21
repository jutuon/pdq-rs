fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .flag_if_supported("-Wno-deprecated-declarations")
        .include("vendor")
        .include("vendor/pdq/cpp")
        .file("vendor/pdq_wrapper.cpp")
        .file("vendor/pdq/cpp/common/pdqhashtypes.cpp")
        .file("vendor/pdq/cpp/common/pdqhamming.cpp")
        .file("vendor/pdq/cpp/common/pdqutils.cpp")
        .file("vendor/pdq/cpp/hashing/pdqhashing.cpp")
        .file("vendor/pdq/cpp/hashing/torben.cpp")
        .file("vendor/pdq/cpp/downscaling/downscaling.cpp")
        .compile("pdq-cpp");

    println!("cargo:rerun-if-changed=vendor");
}

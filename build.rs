fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/libfoo.cpp")
        .flag_if_supported("-std=c++14")
        .compile("test_cxx");
}

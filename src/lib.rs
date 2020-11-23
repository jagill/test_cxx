#[cxx::bridge(namespace=my_namespace)]
pub mod ffi {
    extern "Rust" {
        fn bar() -> i32;
    }
}

pub fn bar() -> i32 {
    5
}

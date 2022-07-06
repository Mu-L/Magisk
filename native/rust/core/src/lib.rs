pub use base;
pub use logging::*;

mod logging;

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        fn rust_test_entry();
    }
}

#[cxx::bridge(namespace = "rust")]
pub mod ffi2 {
    extern "Rust" {
        fn android_logging();
        fn magisk_logging();
        fn zygisk_logging();
    }
}

fn rust_test_entry() {}

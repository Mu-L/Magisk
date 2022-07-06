#![feature(format_args_nl)]

pub use logging::*;
pub use misc::*;

mod logging;
mod misc;

#[cxx::bridge]
pub mod ffi {
    pub enum LogLevel {
        Error,
        Warn,
        Info,
        Debug,
    }

    extern "Rust" {
        fn log_with_rs(level: LogLevel, msg: &str);
    }
}

#[cxx::bridge(namespace = "rust")]
pub mod ffi2 {
    extern "Rust" {
        fn cmdline_logging();
        fn exit_on_error(b: bool);
    }
}

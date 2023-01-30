#[cfg(all(target_arch = "wasm32", feature = "debug"))]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[cfg(all(target_arch = "wasm32", feature = "debug"))]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_impl(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    pub fn warn_impl(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = trace)]
    pub fn trace_impl(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = trace)]
    pub fn error_impl(s: &str);
}

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! log {
    ($($t:tt)*) => (
        #[cfg(feature = "debug")]
        $crate::logger::log_impl(&format_args!($($t)*).to_string())
    )
}

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! warn {
    ($($t:tt)*) => (
        #[cfg(feature = "debug")]
        $crate::logger::warn_impl(&format_args!($($t)*).to_string())
    )
}

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! trace {
    ($($t:tt)*) => (
        #[cfg(feature = "debug")]
        $crate::logger::trace_impl(&format_args!($($t)*).to_string())
    )
}

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! error {
    ($($t:tt)*) => (
        #[cfg(feature = "debug")]
        $crate::logger::error_impl(&format_args!($($t)*).to_string())
    )
}

/// Logs a message to the console
#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! log {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

/// Logs a warning to the console
#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! warn {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

/// Logs a trace to the console
#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! trace {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

/// Logs an error to the console
#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! error {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

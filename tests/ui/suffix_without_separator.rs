//@run-rustfix
//@aux-build:proc_macros.rs:proc-macro
#![allow(unused)]
#![warn(clippy::suffix_without_separator)]

#[macro_use]
extern crate proc_macros;

fn main() {
    _ = 1_u32;
    _ = 1_u8;
    _ = (1_u8);
    _ = [1_u8, 2_u8];
    _ = 1.0_f64;
    _ = 1_f64;
    // Do not lint
    _ = 1u32;
    _ = 1u8;
    _ = (1u8);
    _ = [1u8, 2u8];
    _ = 1.0f64;
    _ = 1f64;
    external! {
        _ = 1_u32;
        _ = 1_u8;
        _ = (1_u8);
        _ = [1_u8, 2_u8];
        _ = 1.0_f64;
        _ = 1_f64;
    }
    with_span! {
        span
        _ = 1_u32;
        _ = 1_u8;
        _ = (1_u8);
        _ = [1_u8, 2_u8];
        _ = 1.0_f64;
        _ = 1_f64;
    }
}

//@run-rustfix
//@aux-build:proc_macros.rs:proc-macro
#![allow(clippy::inconsistent_digit_grouping, unused)]
#![warn(clippy::suffix_with_separator)]

#[macro_use]
extern crate proc_macros;

fn main() {
    _ = 1u32;
    _ = 1u8;
    _ = (1u8);
    _ = [1u8, 2u8];
    _ = 1.0f64;
    _ = 1f64;
    // Do not lint
    _ = 1_u32;
    _ = 1_u8;
    _ = (1_u8);
    _ = [1_u8, 2_u8];
    _ = 1.0_f64;
    _ = 1__f64;
    _ = 1__u32;
    _ = 1__u8;
    _ = (1__u8);
    _ = [1__u8, 2__u8];
    _ = 1.0__f64;
    _ = 1__f64;
    external! {
        _ = 1u32;
        _ = 1u8;
        _ = (1u8);
        _ = [1u8, 2u8];
        _ = 1.0f64;
        _ = 1f64;
    };
    with_span! {
        span
        _ = 1u32;
        _ = 1u8;
        _ = (1u8);
        _ = [1u8, 2u8];
        _ = 1.0f64;
        _ = 1f64;
    };
}

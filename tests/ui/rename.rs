//! Test for Clippy lint renames.
// run-rustfix

#![allow(dead_code)]
// allow the new lint name here, to test if the new name works
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::block_in_if_condition_expr)]
#![allow(clippy::block_in_if_condition_stmt)]
#![allow(clippy::box_collection)]
#![allow(clippy::blocks_in_if_conditions)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::for_loop_over_fallibles)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::invisible_characters)]
#![allow(clippy::single_char_add_str)]
#![allow(clippy::match_result_ok)]
// uplifted lints
#![allow(invalid_value)]
#![allow(array_into_iter)]
#![allow(unused_labels)]
#![allow(drop_bounds)]
#![allow(temporary_cstring_as_ptr)]
#![allow(non_fmt_panics)]
#![allow(unknown_lints)]
#![allow(invalid_atomic_ordering)]
#![allow(enum_intrinsics_non_enums)]
// warn for the old lint name here, to test if the renaming worked
#![warn(clippy::cyclomatic_complexity)]
#![warn(clippy::mem_discriminant_non_enum)]
#![warn(clippy::option_and_then_some)]
#![warn(clippy::box_vec)]
#![warn(clippy::option_map_unwrap_or)]
#![warn(clippy::option_map_unwrap_or_else)]
#![warn(clippy::result_map_unwrap_or_else)]
#![warn(clippy::option_unwrap_used)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::option_expect_used)]
#![warn(clippy::result_expect_used)]
#![warn(clippy::for_loop_over_option)]
#![warn(clippy::for_loop_over_result)]
#![warn(clippy::identity_conversion)]
#![warn(clippy::zero_width_space)]
#![warn(clippy::single_char_push_str)]
#![warn(clippy::if_let_some_result)]
// uplifted lints
#![warn(clippy::invalid_ref)]
#![warn(clippy::into_iter_on_array)]
#![warn(clippy::unused_label)]
#![warn(clippy::drop_bounds)]
#![warn(clippy::temporary_cstring_as_ptr)]
#![warn(clippy::panic_params)]
#![warn(clippy::unknown_clippy_lints)]
#![warn(clippy::invalid_atomic_ordering)]
#![warn(clippy::mem_discriminant_non_enum)]

#[warn(clippy::stutter)]
fn main() {}

#[warn(clippy::new_without_default_derive)]
struct Foo;

#[warn(clippy::const_static_lifetime)]
fn foo() {}

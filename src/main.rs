#![allow(incomplete_features)]
#![feature(array_methods)]
#![feature(const_str_from_utf8_unchecked)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_fn_transmute)]
#![feature(const_mut_refs)]
#![feature(const_heap)]
#![feature(const_ptr_offset)]
#![feature(const_ptr_write)]
#![feature(const_raw_ptr_deref)]
#![feature(const_panic)]
#![feature(const_eval_limit)]
#![feature(const_ptr_read)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(inline_const)]
#![const_eval_limit = "1000000000"]

use crate::brainfuck::execute;

mod const_vec;
mod brainfuck;

fn main() {
    // Unfortunately inline const block here breaks the compiler :(
    const RESULT: &str = execute(include_str!("../rot13_bc.txt"), "Hello world!");
    println!("{}", RESULT);
}




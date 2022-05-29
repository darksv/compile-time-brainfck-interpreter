#![allow(incomplete_features)]
#![feature(array_methods)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_mut_refs)]
#![feature(const_heap)]
#![feature(const_ptr_write)]
#![feature(const_eval_limit)]
#![feature(const_ptr_read)]
#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(inline_const)]
#![feature(adt_const_params)]
#![feature(const_eval_select)]
#![const_eval_limit = "0"]

use crate::brainfuck::{Op, parse, run};

mod const_vec;
mod brainfuck;

fn main() {
    const CODE: &'static str = include_str!("../rot13_bc.txt");
    const PROGRAM: &[Op] = parse(CODE);
    println!("{}", const { run::<PROGRAM, "Hello world!">() });
}

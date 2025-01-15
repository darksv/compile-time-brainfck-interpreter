#![allow(incomplete_features)]
#![allow(internal_features)]
#![feature(const_heap)]
#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(const_eval_select)]

use crate::brainfuck::{Op, parse, run};

mod const_vec;
mod brainfuck;

fn main() {
    const CODE: &'static str = include_str!("../rot13_bc.txt");
    const PROGRAM: &[Op] = parse(CODE);
    println!("{}", const { run(PROGRAM, "Hello world") });
}

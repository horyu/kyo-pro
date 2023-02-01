#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    for i in (0..=n).rev() {
        println!("{}", i);
    }
}
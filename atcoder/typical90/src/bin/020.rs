#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
    };
    // log2(a) < b*log2(c)
    let tf = a < c.pow(b as u32);
    println!("{}", ["No", "Yes"][tf as usize]);
}

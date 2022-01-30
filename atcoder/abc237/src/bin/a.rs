#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: i128
    };
    let range = -(2i128.pow(31))..(2i128.pow(31));
    let tf = range.contains(&n);
    println!("{}", ["No", "Yes"][tf as usize]);
}

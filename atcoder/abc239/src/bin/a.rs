#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: f64
    };
    let rs = (h * (h + 12800000.0)).sqrt();
    println!("{rs}");
}

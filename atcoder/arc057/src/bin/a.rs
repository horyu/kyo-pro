#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    u128,
};

fn main() {
    input! {
        mut a: u128,
        k: u128,
    };
    if k == 0 {
        println!("{}", 2000000000000 - a);
        return;
    }
    let mut rs = 0;
    while a < 2000000000000 {
        rs += 1;
        a += 1 + k * a;
    }
    println!("{}", rs);
}

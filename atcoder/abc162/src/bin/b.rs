#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    let f = |k: usize| (k + (n - n % k)) * (n / k) / 2;
    println!("{}", (1 + n) * n / 2 - f(3) - f(5) + f(15));
}

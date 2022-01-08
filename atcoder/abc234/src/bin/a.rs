#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        t: isize
    };
    let f = |x| x * x + 2 * x + 3;
    let rs = f(f(f(t) + t) + f(f(t)));
    println!("{}", rs);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    };
    let x = c * a.div_ceil(&c);
    println!("{}", if x <= b { x } else { -1 });
}

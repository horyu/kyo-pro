#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut v: [usize; 4]
    };
    v.sort_unstable();
    let tf = (v[0] + v[1] + v[2] == v[3]) || (v[0] + v[3] == v[1] + v[2]);
    println!("{}", ["No", "Yes"][tf as usize]);
}

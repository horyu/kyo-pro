#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut abc: [usize; 3]
    };
    abc.dedup();
    println!("{}", ["No", "Yes"][(abc.len() == 1) as usize]);
}

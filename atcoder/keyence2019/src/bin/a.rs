#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut nn: [u8; 4]
    };
    nn.sort_unstable();
    println!("{}", ["NO", "YES"][(nn == [1, 4, 7, 9]) as usize]);
}

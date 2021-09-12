#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: Usize1,
        s: Chars
    };
    println!("{}", ["No", "Yes"][(s[n] == 'o') as usize]);
}

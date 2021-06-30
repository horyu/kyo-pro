#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        mut s: Chars,
        abcd: [usize; 4]
    };
    for i in abcd.into_iter().rev() {
        s.insert(i, '"');
    }
    let rs = s.iter().collect::<String>();
    println!("{}", rs);
}

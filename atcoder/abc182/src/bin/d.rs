#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut max = 0isize;
    let mut max_add = 0;
    let mut current_add = 0;
    let mut last_pos = 0;
    for a in aa {
        current_add += a;
        max_add = max_add.max(current_add);
        max = max.max(last_pos + max_add);
        last_pos += current_add;
    }
    println!("{}", max);
}

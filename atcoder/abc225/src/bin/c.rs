#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        bbb: [[i128; m]; n]
    };
    for i in 0..(n - 1) {
        if bbb[i + 1][0] - bbb[i][0] != 7 {
            println!("No");
            return;
        }
    }
    let tf = bbb
        .iter()
        .all(|bb| bb.iter().tuple_windows().all(|(bx, by)| by - bx == 1))
        && !((bbb[0][0] % 7 == 0 && m > 1) || (8 - bbb[0][0] % 7 < m as i128));
    println!("{}", ["No", "Yes"][tf as usize]);
}

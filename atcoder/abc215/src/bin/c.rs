#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    };
    s.sort_unstable();
    let len = s.len();
    let mut cnt = 0;
    let mut viewed = HashSet::new();
    for s in s.into_iter().permutations(len) {
        let s = s.into_iter().collect::<String>();
        if !viewed.contains(&s) {
            cnt += 1;
            if cnt == k {
                println!("{}", s);
                return;
            }
            viewed.insert(s);
        }
    }
}

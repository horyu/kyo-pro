#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut xxx: [Chars; n]
    };
    xxx.insert(0, vec!['.'; 9]);
    let mut count = 0;
    for (mae, ato) in xxx.iter().tuple_windows() {
        for (m, a) in mae.iter().zip(ato.iter()) {
            count += match (m, a) {
                (_, 'x') => 1,
                ('o', _) => 0,
                (_, 'o') => 1,
                _ => 0,
            };
        }
    }
    println!("{}", count);
}

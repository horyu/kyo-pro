#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    };
    let ss = s
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| if c == '0' { Some(i) } else { None })
        .collect_vec();
    let tt = t
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| if c == '0' { Some(i) } else { None })
        .collect_vec();
    if ss.len() != tt.len() {
        println!("-1");
    } else {
        let rs = std::iter::zip(ss, tt).filter(|(i, j)| i != j).count();
        println!("{rs}");
    }
}

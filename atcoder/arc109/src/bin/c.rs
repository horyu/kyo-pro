#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars
    };
    let s = s
        .into_iter()
        .map(|c| match c {
            'R' => 0,
            'S' => 1,
            _ => 2, // 'P'
        })
        .collect_vec();
    let rs = match f(&s, k, 0, &mut HashMap::new()) {
        0 => 'R',
        1 => 'S',
        _ => 'P',
    };
    println!("{rs}");
}

fn f(s: &[usize], d: usize, i: usize, hm: &mut HashMap<(usize, usize), usize>) -> usize {
    let i = i % s.len();
    if let Some(&v) = hm.get(&(d, i)) {
        return v;
    }
    if d == 0 {
        return s[i];
    }
    let v = [[0, 0, 2], [0, 1, 1], [2, 1, 2]][f(s, d - 1, i * 2, hm)][f(s, d - 1, i * 2 + 1, hm)];
    hm.insert((d, i), v);
    v
}

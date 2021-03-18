#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    if n <= 2 {
        println!("0");
        return;
    }
    let mut counts = [0usize; 3];
    for c in &s {
        counts[c_to_i(&c)] += 1;
    }
    if counts.contains(&0) {
        println!("0");
        return;
    }
    let mut rs = counts.iter().product::<usize>();
    for i in 0..(n - 2) {
        let si = s[i];
        for j in (i + 1)..(n - 1) {
            let k = 2 * j - i;
            if k >= n {
                break;
            }
            let sj = s[j];
            let sk = s[k];
            if (si != sj) && (sk != si) && (sk != sj) {
                rs -= 1;
            }
        }
    }
    println!("{}", rs);
}

fn c_to_i(c: &char) -> usize {
    match c {
        'R' => 0usize,
        'G' => 1usize,
        'B' => 2usize,
        _ => unreachable!(),
    }
}

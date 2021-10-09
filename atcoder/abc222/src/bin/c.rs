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
        aa: [Chars; 2 * n]
    };
    let mut ranks = (0..(2 * n)).collect_vec();
    let mut wins = vec![0usize; 2 * n];
    for j in 0..m {
        for (&x, &y) in ranks.iter().tuples() {
            let ax = aa[x][j];
            let ay = aa[y][j];
            match (ax, ay) {
                ('G', 'C') | ('C', 'P') | ('P', 'G') => {
                    wins[x] += 1;
                }
                ('C', 'G') | ('P', 'C') | ('G', 'P') => {
                    wins[y] += 1;
                }
                _ => (),
            }
        }
        ranks.sort_unstable_by(|&x, &y| wins[y].cmp(&wins[x]).then(x.cmp(&y)));
        // eprintln!("*{}", wins.iter().join(" "));
        // eprintln!("-{}", ranks.iter().join(" "));
    }
    println!("{}", ranks.into_iter().map(|r| r + 1).join("\n"));
}

#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut bb = vec![0; n];
    for (i, a) in aa.iter().enumerate().rev() {
        if *a != bb[i..].iter().step_by(i + 1).sum::<usize>() % 2 {
            bb[i] = 1;
        }
    }
    let rs = bb
        .iter()
        .enumerate()
        .filter(|ib| *ib.1 > 0)
        .map(|ib| ib.0 + 1)
        .collect_vec();
    if rs.is_empty() {
        println!("0");
    } else {
        println!("{}\n{}", rs.len(), rs.iter().join(" "));
    }
}

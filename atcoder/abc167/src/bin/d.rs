#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [Usize1; n]
    };
    let mut here = 0;
    let mut hist = vec![here];
    let mut visited = vec![false; n];
    visited[here] = true;
    for count in 1..=k {
        here = aa[here];
        hist.push(here);
        if visited[here] {
            if let Some(loop_start) = hist[..count].iter().position(|&pos| pos == here) {
                let loop_size = count - loop_start;
                let fuel = (k - count) % loop_size;
                // dbg!(&hist);
                // dbg!(&loop_start, &count, &fuel, &here);
                for _ in 0..fuel {
                    here = aa[here];
                }
                // dbg!(hist);
                println!("{}", here + 1);
                return;
            }
        }
        visited[here] = true;
    }
    println!("{}", here + 1);
}

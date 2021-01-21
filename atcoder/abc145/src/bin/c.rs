#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let mut sum = 0.0;
    let mut count = 0;
    let mut memo = vec![vec![0.0; n]; n];
    for indexes in (0..n).permutations(n) {
        count += 1;
        sum += indexes.windows(2).fold(0.0, |acc, pair| {
            let mae = pair[0];
            let ato = pair[1];
            if memo[mae][ato] == 0.0 {
                let m = xxyy[mae];
                let a = xxyy[ato];
                memo[mae][ato] = (((m.0 - a.0).pow(2) + (m.1 - a.1).pow(2)) as f64).sqrt();
                memo[ato][mae] = memo[mae][ato];
            }
            acc + memo[mae][ato]
        });
    }
    println!("{}", (sum / (count as f64)));
}

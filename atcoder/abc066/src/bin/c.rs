#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    if n == 1 {
        println!("{}", aa[0]);
        return;
    }
    // 1 -> 0
    // 2 -> 1 0
    // 3 -> 2 0 1
    // 4 -> 3 1 0 2
    // 5 -> 4 2 0 1 3
    // 6 -> 5 3 1 0 2 4
    let evens = (0..(n + 1) / 2)
        .map(|i: usize| aa[2 * i])
        .collect::<Vec<usize>>();
    let odds = (0..n / 2)
        .map(|i: usize| aa[2 * i + 1])
        .collect::<Vec<usize>>();
    let (mae, ato) = if n % 2 == 0 {
        (odds, evens)
    } else {
        (evens, odds)
    };
    println!(
        "{}",
        mae.iter()
            .rev()
            .chain(ato.iter())
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };
    let range = a..=b;
    println!(
        "{}",
        (1..=n)
            .filter(|&i| {
                let mut sum = 0;
                let mut j = i;
                while j > 0 {
                    sum += j % 10;
                    j /= 10;
                }
                range.contains(&sum)
            })
            .sum::<usize>()
    );
}

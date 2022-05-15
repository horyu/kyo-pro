#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    };
    for size in divisors(n) {
        let mut diff = 0;
        for i in 0..size {
            let mut hm = HashMap::new();
            for j in 0..(n / size) {
                *hm.entry(s[(i + j * size)]).or_insert(0) += 1;
            }
            diff += n / size - hm.values().max().unwrap();
        }
        if diff <= k {
            println!("{size}");
            return;
        }
    }
}

// 公約数 https://qiita.com/Cassin01/items/9bc63a4bde5526150681
fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort_unstable();
    divisors
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        k: usize,
    };
    let a_max = k.nth_root(3);
    let mut rs = 0usize;
    let dd = divisors(k);
    for (i, &a) in dd.iter().enumerate() {
        if a_max < a {
            break;
        }
        let bc = k / a;
        for &b in &dd[i..] {
            if bc < b * b {
                break;
            }
            let c = bc / b;
            if a * b * c == k {
                rs += 1;
            }
        }
    }

    println!("{rs}");
}

// https://qiita.com/Cassin01/items/9bc63a4bde5526150681
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

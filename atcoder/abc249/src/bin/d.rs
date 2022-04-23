#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hm = HashMap::new();
    for &a in &aa {
        *hm.entry(a).or_insert(0) += 1usize;
    }
    let mut rs = 0;
    for (&k, &c) in &hm {
        for d in half_divisors(k) {
            let d_c = hm.get(&d).unwrap_or(&0);
            let dd = k / d;
            if d == dd {
                rs += d_c * d_c * c;
            } else {
                let dd_c = hm.get(&dd).unwrap_or(&0);
                rs += d_c * dd_c * 2 * c;
            }
        }
    }
    println!("{rs}");
}

// https://qiita.com/Cassin01/items/9bc63a4bde5526150681
fn half_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        if n % i == 0 {
            divisors.push(i);
            // if i != n / i {
            //     divisors.push(n / i);
            // }
        }
    }
    // divisors.sort_unstable();

    divisors
}

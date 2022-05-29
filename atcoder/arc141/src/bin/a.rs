#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_log)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        t: usize,
        nn: [usize; t]
    };
    for n in nn {
        let log10 = n.log10() as usize;
        if log10 == 1 {
            let rs = (1..=9).map(|x| x * 11).rfind(|&x| x <= n).unwrap();
            println!("{rs}");
            continue;
        }
        // 1桁小さい 9..9
        let mut rs = (0..log10).fold(0, |acc, _| acc * 10 + 9);
        let mut dd = divisors(log10 + 1);
        dd.pop(); // 同じ長さは無視
        for d in dd {
            let m = n / 10usize.pow((log10 + 1 - d) as u32);
            if let Some(t) = [m, m - 1]
                .iter()
                .map(|mm| {
                    (0..((log10 + 1) / d)).fold(0usize, |acc, _| acc * 10usize.pow(d as u32) + mm)
                })
                .find(|&tmp| tmp <= n)
            {
                rs = rs.max(t);
            }
        }
        println!("{rs}");
    }
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

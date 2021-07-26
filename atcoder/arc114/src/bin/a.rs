#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xx: [usize; n]
    };
    let yy = xx.into_iter().map(divisors).collect_vec();
    let primes = [2usize, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut oks = vec![];
    for size in 1..=primes.len() {
        for selected in primes.iter().combinations(size) {
            let hs: HashSet<_> = selected.iter().collect();
            if yy.iter().all(|y| y.iter().any(|i| hs.contains(&i))) {
                let mut ok = 1usize;
                for j in hs {
                    ok *= **j;
                }
                oks.push(ok);
            }
        }
    }
    println!("{}", oks.iter().min().unwrap());
}

// https://qiita.com/Cassin01/items/9bc63a4bde5526150681
fn divisors(n: usize) -> Vec<usize> {
    if n <= 3 {
        return vec![n];
    }
    let mut divisors = Vec::new();
    // n := i * x とおくと、 i が i > root(n) の時、　i はすでに ある x に探索されているから
    // i <= root(n) まで探索すればよい
    for i in 2..=(f64::sqrt(n as f64) + 1e-9) as usize {
        // i で n が割り切れた場合
        if n % i == 0 {
            // 約数リストに格納
            divisors.push(i);

            // n := i * x の x を格納。ただし x := i の時は除く
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort_unstable();
    divisors.push(n);
    divisors
}

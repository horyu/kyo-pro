#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    // N = n * (2 * a1 + (n - 1) * d) / 2
    // a1 -> a, d -> 1
    // 0 = nn + (2a-1)n - 2N
    // a = (-nn + n + 2N) / 2n
    // a = (1 - n) / 2 + N / n
    let mut rs = 0;
    for i in divisors(n) {
        if i % 2 != 0 {
            rs += 1;
        }
    }
    println!("{}", rs);
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
    divisors.sort();
    divisors
}

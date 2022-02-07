#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut x: usize,
        m: usize,
    };
    let mut used = vec![std::usize::MAX; m + 1];
    let mut k = 0;
    let mut sum = 0;
    let mut ss = vec![0];
    while k < n && used[x] == std::usize::MAX {
        sum += x;
        ss.push(sum);
        used[x] = k;
        x = x * x % m;
        k += 1;
    }
    if x != 0 && k != n {
        // 0以外になるループが発生している
        let loop_begin = used[x];
        let loop_size = k - loop_begin;
        let loop_sum = ss[k] - ss[loop_begin];
        sum += (n - k) / loop_size * loop_sum;
        sum += ss[loop_begin + (n - k) % loop_size] - ss[loop_begin];
    }
    println!("{sum}");
}

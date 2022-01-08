#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

const MOD: usize = 1e9 as usize + 7;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    // https://drken1215.hatenablog.com/entry/2018/06/08/210000
    let mut com = vec![[0usize; 2000]; 2000];
    com[0][0] = 1;
    for i in 1..2000 {
        com[i][0] = 1;
        for j in 1..2000 {
            com[i][j] = (com[i - 1][j - 1] + com[i - 1][j]) % MOD;
        }
    }

    for i in 1..=k {
        // n-k 個の赤玉が置いてあり隙間の数は n-k+1 個ある
        // i個の隙間を選択して、それらにk個の青玉を配置する
        let rs = if n - k + 1 < i {
            0
        } else {
            // i個の隙間の選択 (n-k+1) C i
            // i個の隙間にそれぞれ1個ずつ青玉を配置し
            // 余った k-i 個を隙間に適当に割り当てる
            // i H (k-i) = (k-1) C (k-i)
            com[n - k + 1][i] * com[k - 1][k - i] % MOD
        };
        println!("{}", rs);
    }
}

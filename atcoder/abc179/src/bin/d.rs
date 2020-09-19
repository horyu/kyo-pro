#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        llrr: [(usize, usize); k]
    };
    // 距離x移動する方法の数 D(x)
    // D(1) = (S.contains(1) as int)
    // D(2) = D(1) * D(1) + (S.contains(2) as int)
    // D(3) = D(1) * D(2) + D(2) * D(1) + (S.contains(3) as int)
    // D(4) = D(1) * D(3) + D(2) * D(2) + D(3) * D(1) + (S.contains(4) as int)
    // ↑は違う S={1} の場合に D(3)=2 になるのは誤り
    // 
    // 
    // let mut arr = [0usize; 200001];
    // for (l, r) in llrr {
    //     for i in l..=r {
    //         arr[i] = 1;
    //     }
    // }
    // for i in 2..=n {
    //     arr[i] = (1..i).fold(arr[i], |acc, j| acc + arr[j] * arr[i - j] % MOD);
    // }
    // for i in 1..=n {
    //     eprintln!("[{}]{}", i, arr[i]);
    // }
    // println!("{}", arr[n]);
}

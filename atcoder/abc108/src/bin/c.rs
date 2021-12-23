#![allow(unused_imports)]
// use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut rs = 0;
    // k..=n の中でkの倍数の候補から重複有りで3つ選ぶ
    rs += (n / k).pow(3);
    if k % 2 == 0 {
        // i mod k == k/2 のものを足し合わせたらkの倍数になる
        let half_k = k / 2;
        if (n / k) * k + half_k <= n {
            rs += (n / k + 1).pow(3);
        } else {
            rs += (n / k).pow(3);
        }
    }
    println!("{}", rs);
}

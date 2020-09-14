#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::min;

const MOD: i64 = 1000000007;

fn main() {
    input! {
        s: i64
    };
    if s < 6 {
        println!("{}", [0, 1][(s > 2) as usize]);
        return;
    }
    let n = s / 3 + ((s % 3 != 0) as i64);
    // サイズnの数列を右側（最右は {3 or 1 or 2}）から崩していって
    // それより左に割り当てていく
    // 10の場合
    // [3, 3, 3, 1]
    // 0~0 に 10-3*1 を割り当て
    // 0~1 に 10-3*2 を割り当て
    // 0~2 に 10-3*3 を割り当て

    // 3の倍数の場合数列全部が3の1パターン、他は最右が不完全な {1 or 2}なので0
    let mut rs = (s % 3 == 0) as i64;

    // seq_size: 数列のサイズ
    for seq_size in 1..n {
        // 割り当てる数字の合計
        let resource = s - 3 * seq_size;
        // a_{1..=seq_size}にresouceを割り当てる == 区別しない玉を区別する箱に分配
        // http://www.ee.em-net.ne.jp/~takeji/kousiki/kumiwake.pdf
        rs = (rs + combination(seq_size + resource - 1, resource)) % MOD;
    }

    println!("{}", rs);
}

// https://qiita.com/Cassin01/items/d94fc89a97f21af8ac8d
fn extended_euclidean(u: i64, v: i64) -> i64 {
    let mut r0 = u;
    let mut r1 = v;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;
        r0 = r1;
        s0 = s1;
        t0 = t1;
        r1 = r;
        s1 = s;
        t1 = t;
    }
    if t0 < 0 {
        t0 + u
    } else {
        t0
    }
}

// https://qiita.com/Cassin01/items/d94fc89a97f21af8ac8d
// nCc mod m を計算します。
fn combination(mut n: i64, c: i64) -> i64 {
    let mut upe = 1;
    let mut dow = 1;
    for i in 1..=c {
        upe = upe * n % MOD;
        dow = dow * i % MOD;
        n -= 1;
    }
    upe * extended_euclidean(MOD, dow) % MOD
}

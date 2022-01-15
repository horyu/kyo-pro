#![allow(unused_imports)]
#![allow(clippy::many_single_char_names)]
#![feature(format_args_capture)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: i64 = 1e9 as i64 + 7;

#[allow(clippy::int_plus_one)]
fn main() {
    input! {
        x: i64,
        y: i64,
    };
    let mut rs = 0;
    // y = -x + 3m 上
    if (y + x) % 3 == 0 {
        let m = (y + x) / 3;
        let valid_range = m..=(2 * m);
        if valid_range.contains(&y) && valid_range.contains(&x) {
            rs = combination(m, x - m, MOD);
        }
    }
    println!("{rs}");
}

// https://qiita.com/Cassin01/items/d94fc89a97f21af8ac8d
// nCc mod m を計算します。
fn combination(mut n: i64, c: i64, m: i64) -> i64 {
    let mut upe = 1;
    let mut dow = 1;
    for i in 1..c + 1 {
        upe = upe * n % m;
        dow = dow * i % m;

        n -= 1;
    }
    upe * extended_euclidean(m, dow) % m
}

// 拡張ユークリッド互除法
// 入力: 整数 u, v (u > v > 0)
// 返り値: e := v^{-1} mod u を満たす e (=t)
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

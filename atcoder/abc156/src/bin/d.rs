#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };

    let mut rs = mod_exp(2, n, MOD) - 1;
    let sub = combination(n, a, MOD) + combination(n, b, MOD);
    rs = ((rs + MOD) - (sub % MOD)) % MOD;
    println!("{rs}");
}

// https://docs.rs/mod_exp/latest/src/mod_exp/lib.rs.html#37-63
fn mod_exp(base: i64, exponent: i64, modulus: i64) -> i64 {
    let one: i64 = 1;
    let two: i64 = 2;
    let zero: i64 = 0;

    let mut result = one;
    let mut base = base % modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= zero {
            break;
        }

        if exponent % two == one {
            result = (result * base) % modulus;
        }

        exponent >>= one;
        base = (base * base) % modulus;
    }

    result
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

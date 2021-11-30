#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: u128,
        s1: Chars,
        // s2: Chars,
    };
    #[derive(Clone, Copy)]
    enum D {
        Yoko, // =
        Tate, // |
    }
    let s = {
        let mut v = vec![];
        let mut l = 0usize;
        while l < s1.len() {
            match (s1.get(l), s1.get(l + 1)) {
                (Some(x), Some(y)) => {
                    if x == y {
                        v.push(D::Yoko);
                        l += 2;
                    } else {
                        v.push(D::Tate);
                        l += 1;
                    }
                }
                (Some(_), None) => {
                    v.push(D::Tate);
                    l += 1;
                }
                _ => unreachable!(),
            }
        }
        v
    };

    // =|| (3 * 2) * 1 * 2
    // == (3 * 2) * 3
    // 1 2  1 2  1 3
    // 2 1  2 3  2 1
    // |= 3 * 2
    const MOD: u128 = 1000000007;
    let mut rs = match s[0] {
        D::Tate => 3,
        D::Yoko => 6,
    };
    for (x, y) in s.into_iter().tuple_windows() {
        match (x, y) {
            (D::Tate, D::Tate) => rs = (rs * 2) % MOD,
            (D::Tate, D::Yoko) => rs = (rs * 2) % MOD,
            (D::Yoko, D::Tate) => (),
            (D::Yoko, D::Yoko) => rs = (rs * 3) % MOD,
        }
    }
    println!("{}", rs);
}

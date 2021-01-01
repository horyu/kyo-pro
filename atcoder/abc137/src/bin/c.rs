#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    };
    let hm = ss
        .into_iter()
        .map(|mut s| {
            s.sort_unstable();
            (s.into_iter().collect::<String>(), ())
        })
        .into_group_map();
    let rs = hm.values().fold(0usize, |acc, vv| {
        let len = vv.len();
        acc + if len == 1 { 0 } else { combination(len, 2) }
    });
    println!("{}", rs);
}
// nCc を計算
fn combination(n: usize, c: usize) -> usize {
    if n < c {
        panic!("nCr で n < cとなった. n={} c={}", n, c);
    }
    if n == c {
        return 1;
    }
    let c = std::cmp::min(n - c, c);
    let mut rs = 1;
    for bunbo in (n - c + 1)..=n {
        rs *= bunbo;
    }
    for bunsi in 2..=c {
        rs /= bunsi;
    }
    rs
}

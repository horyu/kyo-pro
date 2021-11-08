#![allow(unused_imports)]
#![allow(clippy::many_single_char_names)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        llrr: [(Usize1, Usize1); q]
    };
    let mut vv = vec![0; n + 1];
    for (l, r) in llrr {
        vv[l] += 1;
        vv[r + 1] += 1;
    }
    let mut s = String::with_capacity(n);
    let mut x = 0;
    for v in &vv[0..n] {
        x += v;
        let c = if x % 2 == 0 { '0' } else { '1' };
        s.push(c);
    }
    println!("{}", s);
}

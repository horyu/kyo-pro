#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        llrrtt: [(Usize1, Usize1, usize); q]
    };
    let mut aa = vec![0; n];
    for (l, r, t) in llrrtt {
        for i in l..=r {
            aa[i] = t;
        }
    }
    for a in aa {
        println!("{}", a);
    }
}

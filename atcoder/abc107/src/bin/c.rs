#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut xx: [isize; n]
    };
    let mut pp = vec![0];
    let mut nn = vec![0];
    for x in xx {
        if x < 0 {
            nn.push(x.abs());
        } else {
            pp.push(x);
        }
    }
    pp.sort_unstable();
    nn.sort_unstable();

    let mut rs = std::isize::MAX;
    for (pi, p) in pp.iter().enumerate() {
        if let Some(n) = k.checked_sub(pi).and_then(|ni| nn.get(ni)) {
            rs = rs.min(n * 2 + p);
        }
    }
    for (ni, n) in nn.iter().enumerate() {
        if let Some(p) = k.checked_sub(ni).and_then(|pi| pp.get(pi)) {
            rs = rs.min(p * 2 + n);
        }
    }

    println!("{}", rs);
}

#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x: usize,
        mut aa: [usize; n]
    };
    let mut rs = 0;
    for i in 0..(aa.len() - 1) {
        if aa[i] == 0 {
            continue;
        }
        let diff = (aa[i] + aa[i + 1]).saturating_sub(x);
        if diff > 0 {
            rs += diff;
            // aa[i+1]から優先して引く
            aa[i + 1] = aa[i + 1].saturating_sub(diff);
        }
    }
    rs += aa.last().unwrap().saturating_sub(x);
    println!("{}", rs);
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n]
    };
    aa.insert(0, 0);
    aa.push(0);
    let sum = aa.windows(2).fold(0, |acc, xy| acc + (xy[0] - xy[1]).abs());
    for i in 1..=n {
        let rs = sum - (aa[i - 1] - aa[i]).abs() - (aa[i] - aa[i + 1]).abs()
            + (aa[i - 1] - aa[i + 1]).abs();
        println!("{}", rs);
    }
}

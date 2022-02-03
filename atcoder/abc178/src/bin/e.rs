#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        xxyy: [(isize, isize); n]
    };
    use itertools::MinMaxResult;
    let zz_minmax = xxyy.iter().map(|(x, y)| x + y).minmax();
    let z = match zz_minmax {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => 0,
    };
    let ww_minmax = xxyy.iter().map(|(x, y)| x - y).minmax();
    let w = match ww_minmax {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => 0,
    };
    let rs = z.max(w);
    println!("{rs}");
}

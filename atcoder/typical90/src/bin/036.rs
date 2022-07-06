#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        xxyy: [(isize, isize); n],
        qq: [Usize1; q],
    };
    // 45度回転
    // s=x-y, t=x+y
    let mut smin = std::isize::MAX;
    let mut smax = std::isize::MIN;
    let mut tmin = std::isize::MAX;
    let mut tmax = std::isize::MIN;
    for &(x, y) in &xxyy {
        let s = x - y;
        let t = x + y;
        smin = smin.min(s);
        smax = smax.max(s);
        tmin = tmin.min(t);
        tmax = tmax.max(t);
    }
    for q in qq {
        let (x, y) = xxyy[q];
        let s = x - y;
        let t = x + y;
        let rs = [s - smin, s - smax, t - tmin, t - tmax]
            .into_iter()
            .fold(0, |rs, v| rs.max(v.abs()));
        println!("{rs}")
    }
}

#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut hh: [isize; n],
        ww: [isize; m],
    };
    if n == 1 {
        let h = hh[0];
        let rs = ww.into_iter().map(|w| (w - h).abs()).min().unwrap();
        println!("{rs}");
        return;
    }

    hh.sort_unstable();
    let lsum = hh
        .iter()
        .tuples()
        .map(|(x, y)| y - x)
        .fold(vec![0isize], |mut v, z| {
            v.push(v.last().unwrap() + z);
            v
        });
    let rsum = hh[1..]
        .iter()
        .tuples()
        .map(|(x, y)| y - x)
        .fold(vec![0isize], |mut v, z| {
            v.push(v.last().unwrap() + z);
            v
        });

    let mut rs = std::isize::MAX;
    for w in ww {
        let i = hh.lower_bound(&w) / 2 * 2;
        let tmp = (hh[i] - w).abs() + lsum[i / 2] + rsum[n / 2] - rsum[i / 2];
        rs = rs.min(tmp);
    }
    println!("{rs}");
}

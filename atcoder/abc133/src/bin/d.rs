#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let sum = aa.iter().sum::<usize>();
    let b0 = (sum / 2 - aa[1..].iter().step_by(2).sum::<usize>()) * 2;
    let bb = aa[..(n - 1)].iter().fold(vec![b0], |mut vv, a| {
        vv.push((a - vv.last().unwrap() / 2) * 2);
        vv
    });
    println!("{}", bb.into_iter().join(" "));
}

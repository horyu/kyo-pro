#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut ss: [Chars; n]
    };
    let arr = ['M', 'A', 'R', 'C', 'H'];
    let mut vv = vec![0usize; 5];
    for s in ss {
        if let Some(pos) = arr.iter().position(|x| x == &s[0]) {
            vv[pos] += 1;
        }
    }
    let rs = vv
        .iter()
        .combinations(3)
        .fold(0, |acc, xxx| acc + xxx[0] * xxx[1] * xxx[2]);
    println!("{}", rs);
}

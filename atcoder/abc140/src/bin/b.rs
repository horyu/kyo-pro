#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        bb: [usize; n],
        cc: [usize; n - 1],
    };
    let mut sum = 0;
    let mut pre = n + 1;
    for i in 0..n {
        sum += bb[aa[i]];
        if aa[i] == pre + 1 {
            sum += cc[pre];
        }
        pre = aa[i];
    }
    println!("{}", sum);
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        mut aa: [isize; n]
    };
    aa.sort_unstable();
    // ソートしたら 先頭から
    // -(n-1)* aa[0]* + -(n-3)*aa[1] + .. + (n-3)*aa[n-2] + (n-1)*aa[n-1]
    let mut sum = 0;
    for (i, coefficient) in ((1 - n)..=(n - 1)).step_by(2).enumerate() {
        sum += coefficient * aa[i];
    }
    println!("{}", sum);
}

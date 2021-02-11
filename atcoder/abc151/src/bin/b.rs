#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        aa: [usize; n - 1],
    };
    let sum: usize = aa.iter().sum();
    // (sum + rs) = m * n
    let rs = (m * n).saturating_sub(sum);
    println!("{}", [-1, rs as i32][(rs <= k) as usize]);
}

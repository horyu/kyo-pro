#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    // 基本的に4回毎に1の位はループする
    // b = 4n + m (m=0,1,2,3) とすると
    // b^c = 4(~) + m^c
    let m = b % 4;
    let pow = match m {
        0 => 4,
        1 => 1,
        2 if c == 1 => 2,
        2 => 4,
        3 if c % 2 == 1 => 3,
        3 => 1,
        _ => unreachable!(),
    };
    println!("{}", (a % 10).pow(pow) % 10);
}

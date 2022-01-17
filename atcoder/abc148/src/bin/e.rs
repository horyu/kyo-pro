#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: u128
    };
    if n < 10 || n % 2 == 1 {
        println!("0");
        return;
    }
    let upper = 1e19 as u128;
    let mut rs = 0;
    let mut div = 10;
    while div < upper {
        rs += n / div;
        div *= 5;
    }
    println!("{rs}");
}

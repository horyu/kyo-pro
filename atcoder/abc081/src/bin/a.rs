#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        sss: Chars
    };
    println!("{}", sss.iter().filter(|&&s| s == '1').count());
}

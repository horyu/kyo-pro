#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [usize; 3]
    };
    abc.sort_unstable();
    println!("{}", abc[0] + abc[1]);
}

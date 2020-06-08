#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [usize; 3]
    };
    abc.sort();
    println!("{}", abc[1]);
}

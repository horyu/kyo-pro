#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut ll: [usize; 3]
    };
    ll.sort();
    println!("{}", if ll[0] == ll[1] { ll[2] } else { ll[0] });
}

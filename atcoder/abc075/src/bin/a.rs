#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [i8; 3]
    };
    abc.sort_unstable();
    println!("{}", [abc[0], abc[2]][(abc[0] == abc[1]) as usize]);
}

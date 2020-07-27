#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: String,
        b: String
    };
    // [*2..13, 1].each {|i| print "\"#{i}\", "}
    let arr = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "1",
    ];
    let ai = arr.iter().position(|&num| num == &a).unwrap();
    let bi = arr.iter().position(|&num| num == &b).unwrap();
    println!(
        "{}",
        if ai > bi {
            "Alice"
        } else if ai < bi {
            "Bob"
        } else {
            "Draw"
        }
    );
}

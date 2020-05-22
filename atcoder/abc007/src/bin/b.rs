#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: String
    };
    let len = a.len();
    if a == "a" {
        println!("-1");
    } else if len == 1 {
        let byte = a.into_bytes()[0];
        println!("{}", char::from(byte - 1));
    } else {
        println!("{}", &a[0..(len - 1)]);
    }
}

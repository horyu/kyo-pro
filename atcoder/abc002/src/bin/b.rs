#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        mut w: Chars
    };
    let mut s = String::new();
    for c in w {
        if !"aiueo".contains(c) {
            s.push(c)
        }
    }
    println!("{}", s);
}

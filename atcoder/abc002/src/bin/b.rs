#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        mut w: String
    };
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    w.retain(|c| !vowels.iter().any(|&v| c==v));
    println!("{}", w);
}

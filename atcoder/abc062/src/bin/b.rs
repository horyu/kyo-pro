#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        aaa: [String; h]
    };
    println!("{}", "#".repeat(w + 2));
    for aa in aaa {
        println!("#{}#", aa);
    }
    println!("{}", "#".repeat(w + 2));
}

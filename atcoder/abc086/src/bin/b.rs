#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    let num = format!("{}{}", a, b).parse::<usize>().unwrap();
    for i in 1..=num {
        if i * i == num {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

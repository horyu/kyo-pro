#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut x: usize,
    };
    if x == 2 {
        println!("2");
        return;
    }
    if x % 2 == 0 {
        x += 1;
    }
    let rs = (x..).into_iter().step_by(2).find(is_prime).unwrap();
    println!("{}", rs);
}

fn is_prime(x: &usize) -> bool {
    (3..=((f64::sqrt(*x as f64) + 1e-9) as usize))
        .step_by(2)
        .all(|i| x % i != 0)
}

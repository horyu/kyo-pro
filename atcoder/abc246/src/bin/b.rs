#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    if a == 0 {
        println!("0.0 1.0",);
    } else if b == 0 {
        println!("1.0 0.0",);
    } else {
        let a = a as f64;
        let b = b as f64;
        let r = (a * a + b * b).sqrt();
        println!("{} {}", a / r, b / r);
    }
}

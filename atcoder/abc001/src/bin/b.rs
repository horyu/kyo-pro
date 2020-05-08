#![allow(unused_imports)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        m: i64
    };
    println!(
        "{:02}",
        match m {
            x if x < 100 => 0,
            x if x <= 5000 => x * 10 / 1000,
            x if x <= 35000 => 50 + x / 1000,
            x if x <= 70000 => (x / 1000 - 30) / 5 + 80,
            _ => 89,
        }
    );
}

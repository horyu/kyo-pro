#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        sss: [Chars; 3]
    };
    println!(
        "{}",
        sss.iter()
            .map(|s| s[0].to_uppercase().to_string())
            .collect::<String>()
    );
}

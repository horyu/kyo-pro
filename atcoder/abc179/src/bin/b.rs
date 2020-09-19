#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ddd: [[u8; 2]; n]
    };
    let zoros = ddd.iter().map(|dd| dd[0] == dd[1]).collect::<Vec<bool>>();
    for w in zoros.windows(3) {
        if w[0] && w[1] && w[2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

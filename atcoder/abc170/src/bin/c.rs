#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        n: usize,
        pp: [usize; n]
    };
    let mut diff = 0;
    loop {
        let left = x - diff;
        if !pp.contains(&(left)) {
            println!("{}", left);
            return;
        }
        let right = x + diff;
        if !pp.contains(&(right)) {
            println!("{}", right);
            return;
        }
        diff += 1;
    }
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut ttxxyy: [(isize, isize, isize); n]
    };
    ttxxyy.insert(0, (0, 0, 0));
    for tmp in ttxxyy.windows(2) {
        let a = tmp[0];
        let b = tmp[1];
        let (at, ax, ay) = a;
        let (bt, bx, by) = b;
        let time = bt - at;
        let distance = (bx - ax).abs() + (by - ay).abs();
        if (time < distance) || ((time - distance) % 2 != 0) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

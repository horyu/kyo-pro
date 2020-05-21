#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        m: isize
    };
    //  x +  y +  z = n
    // 2x + 3y + 4z = m
    // x, y, z >= 0
    // -----
    // x =   z - m + 3n
    // y = -2z + m - 2n
    for z in 0..=n {
        let x = z - m + 3 * n;
        let y = -2 * z + m - 2 * n;
        if (x >= 0) && (y >= 0) {
            println!("{} {} {}", x, y, z);
            return;
        }
    }
    println!("-1 -1 -1");
}

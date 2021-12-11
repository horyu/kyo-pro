#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        c: [[isize; 3]; 3]
    };
    let mut a = [0; 3];
    let mut b = [0; 3];
    b[0] = c[0][0];
    b[1] = c[0][1];
    b[2] = c[0][2];
    a[1] = c[1][0] - b[0];
    a[2] = c[2][0] - b[0];

    #[allow(clippy::needless_range_loop)]
    for i in 0..3 {
        for j in 0..3 {
            if a[i] + b[j] != c[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

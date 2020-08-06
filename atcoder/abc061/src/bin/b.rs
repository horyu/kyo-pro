#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut arr = [0; 50];
    for (a, b) in aabb {
        arr[a] += 1;
        arr[b] += 1;
    }
    for i in 0..n {
        println!("{}", arr[i]);
    }
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    if n == 1 {
        println!("1");
        return;
    }
    let mut arr = [0isize; 100_002];
    for a in aa {
        arr[a] += 1;
        arr[a + 1] += 1;
        arr[a + 2] += 1;
    }
    println!("{}", arr.iter().max().unwrap());
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: Usize1
    };
    let mut arr = [0; 1_000_000];
    arr[2] = 1;
    if n >= 3 {
        for i in 3..=n {
            arr[i] = (arr[i - 1] + arr[i - 2] + arr[i - 3]) % 10007;
        }
    }
    println!("{}", arr[n]);
}

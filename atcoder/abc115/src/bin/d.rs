#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

// (0..50).reduce([1]){|acc, i| acc << acc[-1] * 2 + 3}
const SIZES: [usize; 51] = {
    let mut arr = [1usize; 51];
    let mut i = 1;
    while i < 51 {
        arr[i] = arr[i - 1] * 2 + 3;
        i += 1;
    }
    arr
};
const PP: [usize; 51] = {
    let mut arr = [1usize; 51];
    let mut i = 1;
    while i < 51 {
        arr[i] = arr[i - 1] * 2 + 1;
        i += 1;
    }
    arr
};

fn main() {
    input! {
        mut n: usize,
        mut x: usize,
    };
    let mut rs = 0;
    while x != 0 {
        let size = SIZES[n];
        let center = size / 2 + 1;
        if x < center {
            // 左端のパンだけ食べる
            x -= 1;
        } else {
            // 中央パティとその左側すべて食べる
            x -= center;
            rs += 1 + n.checked_sub(1).map(|i| PP[i]).unwrap_or(0);
        }
        if n == 0 {
            break;
        }
        n -= 1;
    }
    println!("{}", rs);
}

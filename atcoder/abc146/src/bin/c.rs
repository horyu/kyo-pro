#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use core::panic;

// use itertools::Itertools;
use proconio::{input, marker::*};

const MAX: i128 = 1000000000;

fn main() {
    input! {
        a: i128,
        b: i128,
        x: i128
    };
    if a + b > x {
        println!("0");
        return;
    }
    let mut right = 1i128;
    while a * right + b * digit(right) <= x {
        right *= 10;
    }
    let mut left = right / 10;
    // https://twitter.com/meguru_comp/status/697008509376835584
    while (right - left).abs() > 1 {
        let mid = (left + right) / 2;
        if a * mid + b * digit(mid) <= x {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left.min(MAX));
}

fn digit(num: i128) -> i128 {
    let mut num = num;
    let mut digit = 0;
    while num != 0 {
        num /= 10;
        digit += 1;
    }
    digit
}

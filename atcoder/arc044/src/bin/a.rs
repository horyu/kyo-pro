#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    if is_prime(n) || is_like_prime(n) {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}

fn is_prime(x: usize) -> bool {
    if x < 2 {
        false
    } else if x == 2 {
        true
    } else if x.is_even() {
        false
    } else {
        (3..=((f64::sqrt(x as f64) + 1e-9) as usize))
            .step_by(2)
            .all(|i| x % i != 0)
    }
}

fn is_like_prime(x: usize) -> bool {
    if x == 1 {
        return false;
    }
    let tmp = x % 10;
    if (tmp == 5) || tmp.is_even() {
        false
    } else {
        let mut y = x;
        let mut tmp = 0;
        while y > 0 {
            tmp += y % 10;
            y /= 10;
        }
        tmp % 3 != 0
    }
}

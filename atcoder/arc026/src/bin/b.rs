#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    let mut sum = if n == 1 { 0 } else { 1 };
    for i in 2..=n.sqrt() {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    use std::cmp::Ordering;
    let rs = match sum.cmp(&n) {
        Ordering::Less => "Deficient",
        Ordering::Equal => "Perfect",
        Ordering::Greater => "Abundant",
    };
    println!("{rs}");
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: usize,
    };
    let mut zero_cnt = 0;
    let mut nums = vec![];
    while n != 0 {
        if n % 10 == 0 {
            zero_cnt += 1;
        } else {
            nums.push(n % 10);
        }
        n /= 10;
    }
    nums.sort_unstable();
    nums.reverse();
    let len = nums.len();
    let mut max = 0;
    for bit in 0..((1 << len) - 1) {
        let indexes = (0..len).filter(|x| (bit & (1 << x)) != 0).collect_vec();
        let mut a = 0;
        let mut b = 0;
        for (i, num) in nums.iter().enumerate() {
            if indexes.contains(&i) {
                a = a * 10 + num;
            } else {
                b = b * 10 + num;
            }
        }
        max = max.max(a * b);
    }
    println!("{}", max * 10usize.pow(zero_cnt));
}

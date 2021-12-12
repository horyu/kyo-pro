#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aa: [usize; n],
        xx: [usize; q],
    };
    aa.sort_unstable();
    for x in xx {
        let mut left = 0;
        let mut right = n;
        while right != left {
            let mid = (left + right) / 2;
            if aa[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        println!("{}", n - left);
    }
}

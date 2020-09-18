#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hm = std::collections::HashMap::new();
    for a in aa {
        *hm.entry(a).or_insert(0) += 1;
    }
    let mut sum = 0;
    for (num, count) in hm {
        if num < count {
            sum += count - num;
        } else if num > count {
            sum += count;
        }
    }
    println!("{}", sum);
}

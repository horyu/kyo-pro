#![allow(unused_imports)]
use std::{collections::BTreeMap, iter::FromIterator};
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [usize; n],
        mut bbcc: [(usize, usize); m]
    };
    aa.sort_unstable();
    bbcc.sort_unstable_by_key(|bc| std::cmp::Reverse(bc.1));
    let mut ai = 0;
    'outer: for (mut b, c) in bbcc {
        while b > 0 {
            if ai >= n || aa[ai] >= c {
                break 'outer;
            }
            aa[ai] = c;
            ai += 1;
            b -= 1;
        }
    }
    println!("{}", aa.iter().sum::<usize>());
}

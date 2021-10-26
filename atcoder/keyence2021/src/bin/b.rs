#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    let mut cnts = vec![0; n + 1];
    for a in aa {
        cnts[a] += 1;
    }
    let mut cnt = cnts[0].min(k);
    let mut rs = 0;
    for (i, c) in cnts.into_iter().enumerate().skip(1) {
        if cnt == 0 {
            break;
        }
        if c < cnt {
            rs += (cnt - c) * i;
        }
        cnt = cnt.min(c);
    }
    println!("{}", rs);
}

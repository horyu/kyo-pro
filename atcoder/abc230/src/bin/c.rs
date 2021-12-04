#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: i128,
        a: i128,
        b: i128,
        p: i128,
        q: i128,
        r: i128,
        s: i128,
    };
    let r1 = (1 - a).max(1 - b)..=((n - a).min(n - b));
    let r2 = (1 - a).max(b - n)..=((n - a).min(b - 1));
    for i in p..=q {
        let mut rs = String::new();
        for j in r..=s {
            let k1 = i - a;
            if (j == b + k1) && r1.contains(&k1) {
                rs.push('#');
                continue;
            }
            let k2 = i - a;
            if (j == b - k2) && r2.contains(&k2) {
                rs.push('#');
                continue;
            }
            rs.push('.');
        }
        println!("{}", rs);
    }
}

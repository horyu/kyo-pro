#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        t: usize,
        mut xxyyzz: [[usize; 3]; t]
    };
    for xyz in xxyyzz {
        let mut rs = std::usize::MAX;
        for i in 0..3 {
            let vv = xyz
                .iter()
                .enumerate()
                .filter_map(|(j, &v)| if i != j { Some(v) } else { None })
                .sorted()
                .collect_vec();
            if vv[0] % 3 == vv[1] % 3 {
                rs = rs.min(vv[1]);
            }
        }
        if rs == std::usize::MAX {
            println!("-1");
        } else {
            println!("{rs}");
        }
    }
}

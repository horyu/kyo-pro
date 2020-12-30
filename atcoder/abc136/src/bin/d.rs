#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars
    };
    s.push('R');
    // LR となったらそれまでを集計
    let mut counts = vec![0; s.len()];
    let mut zure_even = 0;
    let mut zure_odd = 0;
    let mut close_r = 0;
    let mut close_l = 0;
    let mut base_r = 0usize;
    for i in 0..(s.len() - 1) {
        if (i - base_r) % 2 == 0 {
            zure_even += 1;
        } else {
            zure_odd += 1;
        }
        if (s[i], s[i + 1]) == ('R', 'L') {
            close_r = i;
            close_l = i + 1;
        } else if (s[i], s[i + 1]) == ('L', 'R') {
            if (i - close_l) % 2 == 0 {
                counts[close_r] += zure_odd;
                counts[close_l] += zure_even;
            } else {
                counts[close_r] += zure_even;
                counts[close_l] += zure_odd;
            }
            zure_even = 0;
            zure_odd = 0;
            base_r = i + 1;
        }
    }
    let mut rs = counts[0].to_string();
    for count in &counts[1..(counts.len() - 1)] {
        rs.push(' ');
        rs.push_str(&count.to_string())
    }
    println!("{}", rs);
}

#![allow(unused_imports)]
use std::collections::VecDeque;

// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let mut rs = VecDeque::new();
    let mut l = 0;
    for c in s {
        if c == '(' {
            rs.push_back('(');
            l += 1;
        } else {
            if l == 0 {
                rs.push_front('(');
            } else {
                l -= 1;
            }
            rs.push_back(')');
        }
    }
    while l != 0 {
        l -= 1;
        rs.push_back(')');
    }
    println!("{}", rs.iter().collect::<String>());
}

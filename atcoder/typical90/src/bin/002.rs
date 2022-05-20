#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize
    };
    if n.is_odd() {
        println!();
        return;
    }
    // BFS
    // (現在のString, 何回開けられるか、何回閉じられるか)
    let mut rs = vec![];
    let mut qq = VecDeque::new();
    qq.push_back((String::new(), n / 2, 0usize));
    while let Some((s, open, close)) = qq.pop_front() {
        if open + close == 0 {
            rs.push(s);
            continue;
        }
        if open > 0 {
            qq.push_back((format!("{}(", s), open - 1, close + 1));
        }
        if close > 0 {
            qq.push_back((format!("{})", s), open, close - 1));
        }
    }
    for r in rs {
        println!("{r}");
    }
}

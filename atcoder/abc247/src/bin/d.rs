#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        q: usize,
    };
    let mut qq = VecDeque::new();
    for _ in 0..q {
        input! {k: usize};
        if k == 1 {
            input! {x: usize, c: usize};
            qq.push_back((x, c));
        } else {
            input! {c: usize};
            let mut cnt = 0;
            let mut sum = 0;
            while let Some((qx, qc)) = qq.pop_front() {
                cnt += qc;
                sum += qx * qc;
                if cnt >= c {
                    let over = cnt - c;
                    if over > 0 {
                        sum -= qx * over;
                        qq.push_front((qx, over));
                    }
                    break;
                }
            }
            println!("{sum}");
        }
    }
}

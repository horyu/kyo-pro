#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        x: usize,
    };
    if x < 10 {
        println!("{}", x);
        return;
    }

    let mut qq: VecDeque<usize> = (1..=9).collect();
    let mut cnt = 9;
    while let Some(q) = qq.pop_front() {
        let q1 = q % 10;
        for q0 in q1.saturating_sub(1)..=(9.min(q1 + 1)) {
            cnt += 1;
            let num = q * 10 + q0;
            if cnt == x {
                println!("{num}");
                return;
            }
            qq.push_back(num);
        }
    }
}

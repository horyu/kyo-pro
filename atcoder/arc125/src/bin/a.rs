#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [usize; n],
        tt: [usize; m],
    };
    let mut s01 = [false, false];
    for &s in &ss {
        s01[s] = true;
    }
    let mut t01 = [false, false];
    for &t in &tt {
        if !s01[t] {
            println!("-1");
            return;
        }
        t01[t] = true;
    }
    let mut rs = if (ss[0] != tt[0]) || t01.iter().filter(|tf| **tf).count() == 2 {
        // 境目までずらしておく
        let s_first = ss[0];
        let move_cnt = ss
            .iter()
            .position(|s| *s != s_first)
            .unwrap()
            .min(n - ss.iter().rposition(|s| *s != s_first).unwrap());
        if ss[0] == tt[0] {
            move_cnt
        } else {
            move_cnt + 1
        }
    } else {
        1
    };
    for (tx, ty) in tt.into_iter().tuple_windows() {
        if tx == ty {
            rs += 1;
        } else {
            rs += 2;
        }
    }
    println!("{}", rs);
}

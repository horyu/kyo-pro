#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        t: Chars
    };
    fn is_zero(c: char) -> bool {
        c == '0'
    }
    let mut is0cnt = 0;
    let mut is1cnt = 0;
    for i in 0..(n.min(3)) {
        let is0 = is_zero(t[i]);
        if is0 {
            is0cnt += 1;
            if is0cnt >= 2 {
                println!("0");
                return;
            }
        } else {
            is1cnt += 1;
            if is1cnt >= 3 {
                println!("0");
                return;
            }
        }
        for j in (i..n).step_by(3) {
            if is0 != is_zero(t[j]) {
                println!("0");
                return;
            }
        }
    }

    let ranges = 10usize.pow(10u32);
    if t == ['1'] {
        println!("{}", ranges * 2);
    } else if t == ['1', '1'] {
        println!("{}", ranges);
    } else {
        let k = t.iter().filter(|&&c| c == '0').count();
        let rs = if is_zero(t[n - 1]) {
            ranges - k + 1
        } else {
            ranges - k
        };
        println!("{}", rs);
    }
}

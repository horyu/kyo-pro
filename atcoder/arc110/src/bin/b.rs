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
    if n == 1 {
        println!("{}", if is_zero(t[0]) { ranges } else { ranges * 2 });
    } else if n == 2 {
        // 11 or 10 or 01
        println!("{}", if t == ['0', '1'] { ranges - 1 } else { ranges });
    } else {
        // 110 or 101 or 011
        let rs = match (n % 3, &t[0..3]) {
            (0, &['1', '1', '0']) => ranges - n / 3 + 1,
            (2, &['0', '1', '1']) => ranges - n / 3 - 1,
            _ => ranges - n / 3,
        };
        println!("{}", rs);
    }
}

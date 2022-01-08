#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        x: isize
    };
    if x < 100 {
        println!("{}", x);
    } else {
        let mut v: VecDeque<_> = (10isize..=99).map(|w| vec![w / 10, w % 10]).collect();
        while let Some(yy) = v.pop_front() {
            let last = yy[yy.len() - 1];
            if last == 0 {
                continue;
            }
            let diff = yy[1] - yy[0];
            let new_last = last + diff;
            if (0..=9).contains(&new_last) {
                let mut new_yy = yy.clone();
                new_yy.push(new_last);
                let value = new_yy.iter().fold(0, |acc, y| acc * 10 + y);
                if value >= x {
                    println!("{}", value);
                    return;
                }
                v.push_back(new_yy);
            }
        }
    }
}

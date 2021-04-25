#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: String,
    };
    let (mut y, mut m, d) = {
        let arr = s
            .split('/')
            .map(|str| str.parse::<usize>().unwrap())
            .collect_vec();
        (arr[0], arr[1], arr[2])
    };
    for day in get_days(y, m) {
        if (day >= d) && (y % (m * day) == 0) {
            println!("{}/{:02}/{:02}", y, m, day);
            return;
        }
    }
    loop {
        if m == 12 {
            y += 1;
            m = 1;
        } else {
            m += 1;
        }
        for day in get_days(y, m) {
            if y % (m * day) == 0 {
                println!("{}/{:02}/{:02}", y, m, day);
                return;
            }
        }
    }
}

fn get_days(y: usize, m: usize) -> Vec<usize> {
    match m {
        2 => {
            let tf = if y % 400 == 0 {
                true
            } else if y % 100 == 0 {
                false
            } else {
                y % 4 == 0
            };
            if tf {
                (1..=29).collect_vec()
            } else {
                (1..=28).collect_vec()
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => (1..=31).collect_vec(),
        _ => (1..=30).collect_vec(),
    }
}

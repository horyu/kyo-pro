#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mmdd: [String; n]
    };
    let hs: HashSet<_> = mmdd
        .into_iter()
        .map(|md| {
            let mut iter = md
                .split('/')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();

    let mut ttff = vec![false; 366];
    let dd = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut amari = 0;
    let mut youbi = 1; // 土0 日1
    let mut total_days = 0;
    for (m, &max_d) in dd.iter().enumerate() {
        let m = m + 1;
        for d in 1..=max_d {
            amari += hs.contains(&(m, d)) as usize;
            if youbi <= 1 {
                ttff[total_days] = true;
            } else if amari > 0 {
                ttff[total_days] = true;
                amari -= 1;
            }
            youbi = (youbi + 1) % 7;
            total_days += 1;
        }
    }
    let rs = ttff
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .filter_map(|(tf, g)| if tf { Some(g.count()) } else { None })
        .max()
        .unwrap();
    println!("{rs}");
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut bb: VecDeque<isize> = aa.into_iter().sorted().collect();
    let mut dd = bb.clone();
    let mut ee = VecDeque::new();
    let mut cc = VecDeque::new();
    cc.push_back(bb.pop_front().unwrap());
    while let Some(b) = bb.pop_back() {
        cc.push_back(b);
        if let Some (b) = bb.pop_back() {
            cc.push_front(b);
        }
        if let Some (b) = bb.pop_front() {
            cc.push_back(b);
        }
        if let Some (b) = bb.pop_front() {
            cc.push_front(b);
        }
    }
    // eprintln!("{}", cc.iter().join(" "));
    let cc_rs = cc.into_iter().tuple_windows().map(|(x, y)| (x - y).abs()).sum::<isize>();

    ee.push_back(dd.pop_back().unwrap());
    while let Some(d) = dd.pop_front() {
        ee.push_back(d);
        if let Some (d) = dd.pop_front() {
            ee.push_front(d);
        }
        if let Some (d) = dd.pop_back() {
            ee.push_back(d);
        }
        if let Some (d) = dd.pop_back() {
            ee.push_front(d);
        }
    }
    // eprintln!("{}", ee.iter().join(" "));
    let ee_rs = ee.into_iter().tuple_windows().map(|(x, y)| (x - y).abs()).sum::<isize>();

    let rs = cc_rs.max(ee_rs);
    println!("{rs}");
}

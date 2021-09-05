#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

fn main() {
    // solve1(); // 優先度付きキュー
    solve2(); // BTreeMap
}

#[allow(dead_code)]
fn solve1() {
    input! {
        q: usize
    };
    let mut q1 = VecDeque::new();
    let mut q2 = BinaryHeap::new();
    use std::cmp::Reverse;
    for _ in 0..q {
        input! {c: usize};
        match c {
            1 => {
                input! {x: usize};
                q1.push_back(x);
            }
            2 => {
                if let Some(Reverse(v)) = q2.pop() {
                    println!("{}", v);
                } else {
                    println!("{}", q1.pop_front().unwrap());
                }
            }
            3 => {
                while let Some(v) = q1.pop_front() {
                    q2.push(Reverse(v));
                }
            }
            _ => unreachable!(),
        }
    }
}

fn solve2() {
    input! {
        q: usize
    };
    let mut queue = VecDeque::new();
    let mut hm = BTreeMap::new();
    for _ in 0..q {
        input! {c: usize};
        match c {
            1 => {
                input! {x: usize};
                queue.push_back(x);
            }
            2 => {
                if let Some((&k, &v)) = hm.iter().next() {
                    println!("{}", k);
                    if v == 1 {
                        hm.remove(&k);
                    } else {
                        let v = hm.get_mut(&k).unwrap();
                        *v -= 1;
                    }
                } else {
                    println!("{}", queue.pop_front().unwrap());
                }
            }
            3 => {
                while let Some(v) = queue.pop_front() {
                    *hm.entry(v).or_insert(0) += 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

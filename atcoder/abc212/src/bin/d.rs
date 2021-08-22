#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::btree_map::OccupiedEntry;
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    input! {
        q: usize,
    };
    let mut hm = BTreeMap::new();
    let mut plus = 0;
    for _ in 0..q {
        input! {p: usize};
        if p == 3 {
            let mut remove_key = None;
            if let Some((k, v)) = hm.iter_mut().next() {
                println!("{}", k + plus);
                *v -= 1;
                if *v == 0 {
                    remove_key = Some(*k);
                }
            }
            if let Some(k) = remove_key {
                hm.remove(&k);
            }
            continue;
        }
        input! {x: isize};
        if p == 1 {
            *hm.entry(x - plus).or_insert(0) += 1;
        } else if p == 2 {
            plus += x;
        } else {
            unreachable!();
        }
    }
}

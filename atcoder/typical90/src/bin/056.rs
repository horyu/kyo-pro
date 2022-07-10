#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        s: usize,
        aabb: [[usize; 2]; n],
    };
    let vvv = aabb
        .into_iter()
        .map(|mut ab| {
            ab.dedup();
            ab
        })
        .collect_vec();
    let mut hhmm = vec![];
    let mut hm = HashMap::new();
    for &to in &vvv[0] {
        hm.insert(to, 0);
    }
    hhmm.push(hm);
    for (i, vv) in vvv[1..].iter().enumerate() {
        let mut hm = HashMap::new();
        for &from in hhmm[i].keys() {
            for &v in vv {
                let to = from + v;
                if s < to {
                    continue;
                }
                if let hash_map::Entry::Vacant(e) = hm.entry(to) {
                    e.insert(from);
                }
            }
        }
        hhmm.push(hm);
    }
    if hhmm[n - 1].contains_key(&s) {
        let mut from = s;
        let mut rs = VecDeque::new();
        for i in (0..n).rev() {
            let to = from;
            from = *hhmm[i].get(&from).unwrap();
            rs.push_front(if vvv[i][0] == to - from { 'A' } else { 'B' });
        }
        println!("{}", rs.into_iter().join(""));
    } else {
        println!("Impossible");
    }
}

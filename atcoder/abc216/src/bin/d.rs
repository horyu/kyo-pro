#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut aaa = vec![];
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut qq = VecDeque::new();
    for i in 0..m {
        input! {
            k: usize,
            aa: [usize; k],
        };
        let last = *aa.last().unwrap();
        if let Some(e) = hm.get_mut(&last) {
            e.push(i);
            qq.push_back(last);
        } else {
            hm.insert(last, vec![i]);
        }
        aaa.push(aa);
    }

    let mut cnt = 0;
    while let Some(q) = qq.pop_front() {
        cnt += 1;
        if let Some(ii) = hm.remove(&q) {
            for i in ii {
                aaa[i].pop();
                if let Some(&last) = aaa[i].last() {
                    if let Some(e) = hm.get_mut(&last) {
                        e.push(i);
                        qq.push_back(last);
                    } else {
                        hm.insert(last, vec![i]);
                    }
                }
            }
        }
    }
    println!("{}", ["No", "Yes"][(cnt == n) as usize]);
}

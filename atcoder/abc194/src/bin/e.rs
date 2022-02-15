#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::hash_map::Entry;
use std::collections::BTreeSet;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    iter::FromIterator,
};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n]
    };
    let mut bts = BTreeSet::from_iter(0..=n);
    let mut hm = HashMap::new();
    for &a in &aa[0..m] {
        *hm.entry(a).or_insert(0usize) += 1usize;
    }
    for k in hm.keys() {
        bts.remove(k);
    }
    let mut min = *bts.iter().min().unwrap();
    let mut r = m;
    while r < n {
        let ar = aa[r];
        let al = aa[r - m];
        if ar != al {
            if let Entry::Occupied(mut o) = hm.entry(al) {
                let e = o.get_mut();
                *e -= 1;
                if *e == 0 {
                    o.remove_entry();
                    bts.insert(al);
                }
            }

            *hm.entry(ar).or_default() += 1;
            bts.remove(&ar);

            min = min.min(*bts.iter().min().unwrap());
        }
        r += 1;
    }
    println!("{min}");
}

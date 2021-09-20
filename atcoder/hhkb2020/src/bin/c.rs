#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        pp: [usize; n]
    };
    use std::iter::FromIterator;
    let mut hs = std::collections::BTreeSet::from_iter(0..=200_001);
    let mut i = 0;
    for p in pp {
        hs.remove(&p);
        let next = hs.range(i..).next().unwrap();
        println!("{}", next);
        i = *next;
    }
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        ee: [char; 6],
        n: char,
        ll: [char; 6],
    };
    use std::iter::FromIterator;
    let he: HashSet<_> = HashSet::from_iter(ee.iter());
    let hl: HashSet<_> = HashSet::from_iter(ll.iter());
    let intersection = he.intersection(&hl);
    let rs = match intersection.count() {
        6 => 1,
        5 => {
            if hl.contains(&n) {
                2
            } else {
                3
            }
        }
        4 => 4,
        3 => 5,
        _ => 0,
    };
    println!("{}", rs);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut tf = s == t;
    if !tf {
        let mut diffs = vec![];
        for (i, (s, t)) in s.iter().zip(t.iter()).enumerate() {
            if s != t {
                diffs.push((i, *s, *t));
            }
        }
        if diffs.len() == 2
            && diffs[0].0 + 1 == diffs[1].0
            && diffs[0].1 == diffs[1].2
            && diffs[0].2 == diffs[1].1
        {
            tf = true;
        }
    }
    println!("{}", ["No", "Yes"][tf as usize]);
}

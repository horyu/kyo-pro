#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let tf = match s.len().cmp(&t.len()) {
        std::cmp::Ordering::Less => {
            let sg = s
                .into_iter()
                .group_by(|&c| c)
                .into_iter()
                .map(|g| (g.0, g.1.count()))
                .collect_vec();
            let tg = t
                .into_iter()
                .group_by(|&c| c)
                .into_iter()
                .map(|g| (g.0, g.1.count()))
                .collect_vec();
            sg.len() == tg.len()
                && std::iter::zip(sg, tg)
                    .into_iter()
                    .all(|(s, t)| s.0 == t.0 && s.1 <= t.1)
        }
        std::cmp::Ordering::Equal => s == t,
        std::cmp::Ordering::Greater => false,
    };
    println!("{}", ["No", "Yes"][tf as usize]);
}

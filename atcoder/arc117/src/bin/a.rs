#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        b: isize,
    };
    use std::cmp::Ordering;
    let (a_str, b_str) = match a.cmp(&b) {
        Ordering::Equal => ((1..=a).join(" "), (1..=b).map(|b| -b).join(" ")),
        Ordering::Greater => (
            (1..=a).join(" "),
            (1..b)
                .chain(std::iter::once(a * (a + 1) / 2 - b * (b - 1) / 2))
                .map(|b| -b)
                .join(" "),
        ),
        Ordering::Less => (
            (1..a)
                .chain(std::iter::once(b * (b + 1) / 2 - a * (a - 1) / 2))
                .join(" "),
            (1..=b).map(|b| -b).join(" "),
        ),
    };
    println!("{} {}", a_str, b_str);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x1: isize,
        y1: isize,
        r: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize,
    };
    let cricles = vec![
        (x1 - r, y1 - r),
        (x1 - r, y1 + r),
        (x1 + r, y1 - r),
        (x1 + r, y1 + r),
    ];
    println!(
        "{}",
        match cricles
            .into_iter()
            .all(|(x, y)| (x2..=x3).contains(&x) && (y2..=y3).contains(&y))
        {
            false => "YES",
            true => "NO",
        }
    );

    let squeares = vec![(x2, y2), (x2, y3), (x3, y2), (x3, y3)];
    println!(
        "{}",
        match squeares
            .into_iter()
            .all(|(x, y)| (x - x1).pow(2) + (y - y1).pow(2) <= r.pow(2))
        {
            false => "YES",
            true => "NO",
        }
    );
}

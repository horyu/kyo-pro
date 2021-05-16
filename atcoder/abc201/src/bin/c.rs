#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    input! {
        s: Chars
    };
    let rs = (0usize..=9999)
        .filter(|num| {
            let v = [num / 1000, num / 100 % 10, num / 10 % 10, num % 10];
            s.iter().enumerate().all(|(i, c)| match c {
                'o' => v.contains(&i),
                'x' => !v.contains(&i),
                _ => true,
            })
        })
        .count();
    println!("{}", rs);
}

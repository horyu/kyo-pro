#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![feature(vec_retain_mut)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ww: [Chars; n]
    };
    let rs = ww
        .into_iter()
        .map(|mut w| {
            w.retain_mut(|c| {
                *c = match c.to_ascii_lowercase() {
                    'b' | 'c' => '1',
                    'd' | 'w' => '2',
                    't' | 'j' => '3',
                    'f' | 'q' => '4',
                    'l' | 'v' => '5',
                    's' | 'x' => '6',
                    'p' | 'm' => '7',
                    'h' | 'k' => '8',
                    'n' | 'g' => '9',
                    'z' | 'r' => '0',
                    _ => return false,
                };
                true
            });
            w.into_iter().join("")
        })
        .filter(|w| !w.is_empty())
        .join(" ");
    println!("{rs}");
}

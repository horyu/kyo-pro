#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    if let Some(t_i) = s.iter().position(|&c| c.to_ascii_lowercase() == 'i') {
        if let Some(c_i) = s[t_i..].iter().position(|&c| c.to_ascii_lowercase() == 'c') {
            if s[(t_i + c_i)..].iter().any(|&c| c.to_ascii_lowercase() == 't') {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for c in s.chars() {
        print!(
            "{}",
            match c {
                'L' => "<",
                'R' => ">",
                'A' => "A",
                ' ' => " ",
                _ => "",
            }
        )
    }
    println!();
}

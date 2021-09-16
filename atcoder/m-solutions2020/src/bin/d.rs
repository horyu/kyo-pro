#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut cache = 1000usize;
    let mut stock = 0usize;
    for (&x, &y) in aa.iter().tuple_windows() {
        use std::cmp::Ordering;
        match x.cmp(&y) {
            Ordering::Less => {
                if cache >= x {
                    stock += cache / x;
                    cache %= x;
                }
            }
            Ordering::Greater => {
                if stock > 0 {
                    cache += x * stock;
                    stock = 0;
                }
            }
            _ => (),
        }
    }
    if stock != 0 {
        cache += aa.into_iter().last().unwrap() * stock;
    }
    println!("{}", cache);
}

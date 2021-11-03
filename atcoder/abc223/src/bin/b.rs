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
    let min_max = (0..s.len())
        .map(|i| {
            let mut v = s.clone();
            v.rotate_right(i);
            v
        })
        .minmax();
    use itertools::MinMaxResult;
    match min_max {
        MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::OneElement(v) => {
            let rs = v.iter().collect::<String>();
            println!("{}\n{}", rs, rs);
        }
        MinMaxResult::MinMax(min, max) => {
            println!(
                "{}\n{}",
                min.iter().collect::<String>(),
                max.iter().collect::<String>()
            );
        }
    }
    // println!("{}", );
}

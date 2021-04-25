#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        ttaabb: [(u8, usize, usize); q],
    };
    let (mut mae, mut ato) = s.split_at_mut(n);
    for (t, a, b) in ttaabb {
        if t == 1 {
            let a = a - 1;
            let b = b - 1;
            match (a < n, b < n) {
                (true, true) => {
                    mae.swap(a, b);
                }
                (true, false) => {
                    let tmp = mae[a];
                    mae[a] = ato[b % n];
                    ato[b % n] = tmp;
                }
                (false, true) => {
                    let tmp = mae[a % n];
                    mae[a % n] = ato[b];
                    ato[b] = tmp;
                }
                (false, false) => {
                    ato.swap(a % n, b % n);
                }
            }
        } else {
            std::mem::swap(&mut mae, &mut ato);
        }
    }
    println!(
        "{}{}",
        mae.iter().collect::<String>(),
        ato.iter().collect::<String>()
    );
}

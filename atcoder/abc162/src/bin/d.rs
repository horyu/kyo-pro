#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    panic!("漏れ有り");
    input! {
        n: usize,
        s: Chars
    };
    if n <= 2 {
        println!("0");
        return;
    }
    fn c_to_i(c: &char) -> usize {
        match c {
            'R' => 0usize,
            'G' => 1usize,
            'B' => 2usize,
            _ => unreachable!(),
        }
    }
    let mut counts = vec![0; 3];
    for c in &s {
        counts[c_to_i(&c)] += 1;
    }
    if counts.contains(&0) {
        println!("0");
        return;
    }
    let mut rs = 0;
    for i in 0..(n - 2) {
        let si = s[i];
        counts[c_to_i(&si)] -= 1;
        let mut viewed = vec![0; 3];
        for j in (i + 1)..(n - 1) {
            let sj = s[j];
            viewed[c_to_i(&sj)] += 1;
            if si == sj {
                continue;
            }
            let nokori_index = *[0usize, 1, 2]
                .iter()
                .find(|&&i| (i != c_to_i(&si)) && (i != c_to_i(&sj)))
                .unwrap();
            rs += counts[nokori_index] - viewed[nokori_index];
            // j - i == k - j を飛ばす
            // 2j - i == k
            if let Some(&sk) = s.get(2 * j - i) {
                if (sk != si) && (sk != sj) {
                    rs -= 1;
                }
            }
        }
    }
    println!("{}", rs);
}

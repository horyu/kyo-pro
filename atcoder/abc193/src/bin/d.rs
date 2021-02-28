#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    };
    // index => 枚数 のVec
    let ss = chars_to_cards(&s[0..4]);
    let tt = chars_to_cards(&t[0..4]);
    let cards = {
        let mut v = vec![k; 9];
        for (i, (s, t)) in ss.iter().zip(tt.iter()).enumerate() {
            v[i] -= s + t;
        }
        v
    };
    let mut kati = 0usize;
    let mut make = 0usize;
    for s5 in 0..9 {
        for t5 in 0..9 {
            if ((s5 == t5) && (cards[s5] < 2)) || ((cards[s5] < 1) || (cards[t5] < 1)) {
                continue;
            }
            let comb = if s5 == t5 {
                cards[s5] * (cards[s5] - 1)
            } else {
                cards[s5] * cards[t5]
            };
            if score(&ss, s5) > score(&tt, t5) {
                kati += comb;
            } else {
                make += comb;
            }
        }
    }
    println!("{}", kati as f64 / (kati + make) as f64);
}

fn chars_to_cards(x: &[char]) -> Vec<usize> {
    let mut v = vec![0; 9];
    for &c in x {
        v[(c.to_digit(10).unwrap() as usize) - 1] += 1;
    }
    v
}

fn score(xx: &[usize], x5: usize) -> usize {
    let mut score = 0;
    for (i, &maisu) in xx.iter().enumerate() {
        score += (i + 1) * 10usize.pow(maisu as u32 + ((i == x5) as u32));
    }
    score
}

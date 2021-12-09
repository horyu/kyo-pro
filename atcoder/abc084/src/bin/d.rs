#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

const N: usize = 1e5 as usize;

fn main() {
    input! {
        q: usize,
        llrr: [(usize, usize); q],
    };
    let prime_table = sieve_of_eratosthenes(N);
    let ok_table = {
        let mut tbl = vec![false; prime_table.len()];
        for i in (3..prime_table.len()).step_by(2) {
            if prime_table[i] && prime_table[(i + 1) / 2] {
                tbl[i] = true;
            }
        }
        tbl
    };
    let vv = {
        let mut vv = vec![0; ok_table.len() + 1];
        for i in 2..ok_table.len() {
            vv[i] = vv[i - 1] + ok_table[i] as usize;
        }
        vv
    };
    for (l, r) in llrr {
        println!("{}", vv[r] - vv[l - 1]);
    }
}

// https://qiita.com/Cassin01/items/2f90aedded2b8fb017a1#on-%E3%81%AE%E3%82%A8%E3%83%A9%E3%83%88%E3%82%B9%E3%83%86%E3%83%8D%E3%82%B9%E3%81%AE%E7%AF%A9-1
fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut spf = vec![None; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = vec![];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n + 1 {
        if is_prime[i] {
            primes.push(i);
            spf[i] = Some(i);
        }
        for prime in &primes {
            if i * prime >= n + 1 || prime > &spf[i].unwrap() {
                break;
            }
            is_prime[i * prime] = false;
            spf[i * prime] = Some(*prime);
        }
    }
    is_prime
}

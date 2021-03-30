#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize
    };
    let hm = factorization_with_sieve_of_eratosthenes(n);
    let mut rs = 0;
    for count in hm.values() {
        let mut count = *count;
        let mut sub = 1;
        while let Some(new_count) = count.checked_sub(sub) {
            rs += 1;
            sub += 1;
            count = new_count;
        }
    }
    println!("{}", rs);
}

// https://qiita.com/Cassin01/items/2f90aedded2b8fb017a1#%E3%82%A8%E3%83%A9%E3%83%88%E3%82%B9%E3%83%86%E3%83%8D%E3%82%B9%E3%81%AE%E7%AF%A9%E3%82%92%E7%94%A8%E3%81%84%E3%81%9F%E7%B4%A0%E5%9B%A0%E6%95%B0%E5%88%86%E8%A7%A3
fn factorization_with_sieve_of_eratosthenes(mut n: usize) -> HashMap<usize, usize> {
    let primes = sieve_of_eratosthenes(f64::sqrt(n as f64) as usize);
    let mut hm_primes = HashMap::new();
    for prime in primes {
        while n % prime == 0 {
            n /= prime;
            *hm_primes.entry(prime).or_insert(0) += 1;
        }
    }
    if n > 1 {
        *hm_primes.entry(n).or_insert(0) += 1;
    }
    hm_primes
}
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut spf = vec![None; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();
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
    primes
}

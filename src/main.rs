mod trino;

fn print(n: i32) {
    let mut i = 31;
    while i >= 0 {
        if n & (1 << i) == 0 {
            print!("{}", '0');
        } else {
            print!("{}", '1');
        }
        i -= 1;
    }
    println!("")
}
fn add(sum: i32, carry: i32) -> i32 {
    if carry == 0 {
        return sum;
    }
    return add(sum ^ carry, (sum & carry) << 1);
}
fn pow(mut a: i32, mut b: i32) -> i32 {
    let mut res = 1;
    while b != 0 {
        if b & 1 != 0 {
            res = res * a;
        }
        a *= a;
        b >>= 1;
    }
    return res;
}
fn count_primes(n: i32) -> i32 {
    let num: i64 = n as i64;
    let mut primes = vec![i32::MAX; (n / 31) as usize + 1];
    let mut res = 0;
    //将指定位改为0
    primes[0] &= !(1 << 0);
    primes[0] &= !(1 << 1);
    for idx in 2..n {
        if primes[(idx / 31) as usize] & (1 << (idx % 31)) != 0 {
            let mut j = (idx as i64 * idx as i64);
            while j < n as i64 {
                primes[(j / 31) as usize] &= !(1 << (j % 31));
                j += idx as i64;
            }
        }
    }
    for idx in 0..n {
        if primes[(idx / 31) as usize] & (1 << (idx % 31)) != 0 {
            res += 1;
        }
    }
    res
}
use std::collections::{*};
use std::collections::binary_heap::BinaryHeap;

struct Solution {}
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let len = piles.len();
        let mut L = vec![vec!(0; len + 1); len + 1];
        let mut R = vec![vec!(0; len + 1); len + 1];
        for i in 0..len {
            for j in 0..len {
                L[j][i] = piles[i];
            }
        }
        for start in 1..len {
            let mut l = 0;
            let mut r = start;
            while r < len {
                L[l][r] = (piles[l] + R[l + 1][r]).max(piles[r] + R[l][r - 1]);
                R[l][r] = L[l + 1][r].min(L[l][r - 1]);
                r += 1;
                l += 1;
            }
        }
        return L[0][len - 1].max(R[0][len - 1]) == L[0][len - 1];
    }


use regex::Regex;
use std::fs::File;
use std::io::{Read, Seek};
use std::ops::Add;
use rand::Rng;

fn main() {
    Solution::find_subarrays(vec![4,2,4]);
}

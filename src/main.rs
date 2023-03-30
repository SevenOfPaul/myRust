mod trino;
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

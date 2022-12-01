#![feature(binary_heap_into_iter_sorted)]

use std::collections::BinaryHeap;
use std::{env, fs};

use anyhow::Result;

fn max_n(n: usize) -> Result<i32> {
    let Some(path) = env::args().nth(1) else {
        anyhow::bail!("no file provided");
    };

    let values: BinaryHeap<i32> = fs::read_to_string(path)?
        .split("\n\n")
        .map(|list| {
            list.split('\n')
                .filter_map(|nums| nums.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect();

    Ok(values.into_iter_sorted().take(n).sum())
}

fn main() -> Result<()> {
    println!("max is {}", max_n(3)?);
    Ok(())
}

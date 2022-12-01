use std::{env, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let Some(path) = env::args().nth(1) else {
        anyhow::bail!("no file provided");
    };

    let max = fs::read_to_string(path)?
        .split("\n\n")
        .map(|list| {
            list.split('\n')
                .filter_map(|nums| nums.parse::<i32>().ok())
                .sum::<i32>()
        })
        .max()
        .ok_or_else(|| anyhow::anyhow!("invalid input"))?;

    println!("max is {max}");
    Ok(())
}

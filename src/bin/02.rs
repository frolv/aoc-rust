use std::{env, fs, str::FromStr};

use anyhow::{anyhow, Result};

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl Choice {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn compare(&self, other: &Choice) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (_, _) => Outcome::Draw,
        }
    }
}

impl FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(anyhow!("invalid input")),
        }
    }
}

fn calculate_score(contents: &str) -> Result<i32> {
    contents.split('\n').try_fold(0, |sum, line| {
        let choices = line
            .split(' ')
            .map(str::parse::<Choice>)
            .collect::<Result<Vec<_>>>()?;
        let score = choices[1].score() + choices[1].compare(&choices[0]).score();
        Ok(sum + score)
    })
}

fn main() -> Result<()> {
    let Some(path) = env::args().nth(1) else {
        anyhow::bail!("no file provided");
    };

    let contents = fs::read_to_string(path)?;

    let score = calculate_score(&contents)?;
    println!("your score: {score}");
    Ok(())
}

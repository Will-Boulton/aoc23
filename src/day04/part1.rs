use std::collections::{hash_set, HashSet};

pub(crate) fn solve(input: &str) -> usize {
    return input.lines()
        .filter_map(score_line)
        .sum();
}

fn score_line(line:&str) -> Option<usize>
{
    let (card, rest) = line.trim_start_matches("Card ").split_once(':')?;

    let (winners, guesses) = rest.split_once('|')?;

    let winners : HashSet<usize> =
        winners
        .split_whitespace()
        .into_iter()
        .map(|x|x.parse::<usize>().unwrap())
        .collect();

    let mut score = 0usize;

    for n in guesses
        .split_whitespace()
        .into_iter()
        .map(|x|x.parse::<usize>().unwrap())
    {
        if winners.contains(&n) {
            if score == 0 {
                score = 1;
            }
            else {score *= 2;}
        }
    }

    return Some(score)
}

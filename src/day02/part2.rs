use crate::day02::{Game, parse_games};

pub(crate) fn solve(input : &str) -> usize
{
    return parse_games(input)
        .iter()
        .map(|game: &Game| {
            game.max_r * game.max_g * game.max_b
        })
        .sum();
}
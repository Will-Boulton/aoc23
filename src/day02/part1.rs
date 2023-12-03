use crate::day02::{Game, parse_games};

pub(crate) fn solve(input : &str) -> usize
{
    return parse_games(input)
        .iter()
        .filter_map(|game: &Game| {
            if game.max_r <= 12 && game.max_g <= 13 && game.max_b <= 14 {
                return Some(game.id);
            }
            return None;
        })
        .sum();
}


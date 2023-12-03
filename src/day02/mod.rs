mod part1;
mod part2;
mod tests;

use std::cmp::{max};

pub(crate) fn run()
{
    let input = include_str!("input.txt");
    println!("day02 part1 {}", part1::solve(input));
    println!("day02 part2 {}", part2::solve(input));
}



struct Game
{
    id : usize,
    max_r : usize,
    max_g: usize,
    max_b : usize,
}

impl Game{
    fn new(id : usize) -> Game
    {
        return Game{id,max_r:0,max_g:0, max_b:0}
    }
}

fn parse_games(input : &str) -> Vec<Game> {
    return input.lines()
        .map(|line| {
            let (game_id, game_txt) =  line.trim_start_matches("Game ").split_once(":").unwrap();
            let mut game = Game::new(game_id.parse().ok().unwrap());
            game_txt.split(';')
                .for_each(|s|{
                    for itm_ct in s.split(',').map(|c|c.trim()){
                        let (count,colour) = itm_ct.split_once(' ').unwrap();
                        let count = count.parse().ok().unwrap();
                        match colour.as_bytes()[0] {
                            b'r' => { game.max_r = max(count, game.max_r) },
                            b'g' => { game.max_g = max(count, game.max_g) },
                            b'b' => { game.max_b = max(count, game.max_b) },
                            _ => { }
                        }
                    }
                });
            game
        }).collect();
}



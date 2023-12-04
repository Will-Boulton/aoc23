use std::collections::{ HashSet};
use std::ops::Range;
use std::vec::{Vec};
pub(crate) fn solve(input: &str) -> usize {
    let lines : Vec<ParsedLine>=  input.lines()
        .filter_map(parse_line)
        .collect();

    let mut line_counts: Vec<usize> = vec![1;lines.len()];
    return lines.iter().map(|l|collapse(l,&mut line_counts)).sum();
}

fn collapse(line: &ParsedLine, line_counts: &mut Vec<usize>) -> usize{

    let count =  line_counts[line.id];
    for x in line.next_winners() {
        line_counts[x] += count;
    }
    return count;
}


fn parse_line(line: &str) -> Option<ParsedLine>
{  let (card, rest) = line.trim_start_matches("Card").split_once(':')?;

    let card_id = card.trim_start().parse().ok()?;

    let (winners, guesses) = rest.split_once('|')?;

    let winners : HashSet<usize> =
        winners
            .split_whitespace()
            .into_iter()
            .map(|x|x.parse::<usize>().unwrap())
            .collect();


    let guesses = guesses
        .split_whitespace()
        .into_iter()
        .map(|x|x.parse::<usize>().unwrap())
        .collect();

    let winners  = winners.intersection(&guesses).count();
    return Some(ParsedLine::new(card_id,winners))
}
struct ParsedLine {
    id : usize,
    winners : usize
}

impl ParsedLine{
    fn new(id : usize, winners : usize ) -> Self {
        return ParsedLine{id: id-1, winners};
    }

    pub fn next_winners(&self) -> Range<usize> {
        let start = self.id+1;

        return start..start + self.winners;
    }
}
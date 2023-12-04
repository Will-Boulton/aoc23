use crate::macros::run_parts;
mod part1;
mod tests;
mod part2;

pub(crate) fn run()
{
    run_parts!(day04: part1, part2);
}

use crate::macros::run_parts;

mod part1;
mod part2;
mod tests;

pub(crate) fn run()
{
    run_parts!(day01: part1, part2);
}

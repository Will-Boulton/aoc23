mod part1;
mod part2;
mod tests;

pub(crate) fn run()
{
    let input = include_str!("input.txt");
    println!("day01 part1 {}", part1::solve(input));
    println!("day01 part2 {}", part2::solve(input));
}

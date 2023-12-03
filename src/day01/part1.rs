pub fn solve(input: &str) -> usize
{
        let sum_line = |line : &str|{
        let mut digits = line.chars()
            .filter_map(|i|{
            if char::is_digit(i, 10) {
                return Some(i as usize - '0' as usize);
            };
            return None
        }
        );
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        return 10 * first + last;
    };

    return  input.lines()
        .map(|l|
            sum_line(l)
        )
        .sum();
}
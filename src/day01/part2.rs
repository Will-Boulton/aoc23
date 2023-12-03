
const DIGIT_STR : &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn solve(input: &str) -> usize
{
    let sum_line = |line : &str|{
        let mut digits = line
            .chars()
            .enumerate()
            .filter_map(|(i,ch)|{
                if char::is_digit(ch,10) {
                    return Some(ch  as usize - '0' as usize)
                }
                else{
                    return DIGIT_STR
                        .iter()
                        .enumerate()
                        .find_map(|(a,b)|
                            line[i..]
                                .starts_with(b)
                                .then_some(a+1))
                }
            });

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
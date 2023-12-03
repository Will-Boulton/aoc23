use crate::day01;

#[test]
fn part_1()
{
    let example = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

    let result = day01::part1::solve(example);
    assert_eq!(result, 142);
}

#[test]
fn part_2()
{
    let example = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    let result = day01::part2::solve(example);
    assert_eq!(result, 281);
}
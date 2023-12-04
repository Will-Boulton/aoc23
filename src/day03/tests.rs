
    #[test]
    fn part_1()
    {
        let example = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = crate::day03::part1::solve(example);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_1_2()
    {
        {
            let example = r#"...123..."#;

            let result = crate::day03::part1::solve(example);
            assert_eq!(result, 0);
        }
        {
            let example = r#"...123%.."#;

            let result = crate::day03::part1::solve(example);
            assert_eq!(result, 123);
        }
    }

    #[test]
    fn part_1_3()
    {

            let example = r#"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
15.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"#;

            let result = crate::day03::part1::solve(example);
            assert_eq!(result, 925);

    }

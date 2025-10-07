pub mod aoc2022;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let day01_part1 = aoc2022::day01::part01("./test_puzzle_input/day01_test.txt");
        assert_eq!(day01_part1, 24000);
        let day01_part2 = aoc2022::day01::part02("./test_puzzle_input/day01_test.txt");
        assert_eq!(day01_part2, 45000);

        let day02_part1 = aoc2022::day02::part01("./test_puzzle_input/day02_test.txt");
        assert_eq!(day02_part1, 15);
        let day02_part2 = aoc2022::day02::part02("./test_puzzle_input/day02_test.txt");
        assert_eq!(day02_part2, 12);
    }
}

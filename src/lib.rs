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

        let day03_part1 = aoc2022::day03::part01("./test_puzzle_input/day03_test.txt");
        assert_eq!(day03_part1, 157);
        let day03_part2 = aoc2022::day03::part02("./test_puzzle_input/day03_test.txt");
        assert_eq!(day03_part2, 70);

        let day04_part1 = aoc2022::day04::part01("./test_puzzle_input/day04_test.txt");
        assert_eq!(day04_part1, 2);
        let day04_part2 = aoc2022::day04::part02("./test_puzzle_input/day04_test.txt");
        assert_eq!(day04_part2, 4);

        let day05_part1 = aoc2022::day05::part01("./test_puzzle_input/day05_test.txt");
        assert_eq!(day05_part1, "CMZ");
        let day05_part2 = aoc2022::day05::part02("./test_puzzle_input/day05_test.txt");
        assert_eq!(day05_part2, "MCD");

        let day06_part1_test1 = aoc2022::day06::part01("./test_puzzle_input/day06_test.txt");
        assert_eq!(day06_part1_test1, 7);
        let day06_part1_test2 = aoc2022::day06::part01("./test_puzzle_input/day06_test2.txt");
        assert_eq!(day06_part1_test2, 5);
        let day06_part1_test3 = aoc2022::day06::part01("./test_puzzle_input/day06_test3.txt");
        assert_eq!(day06_part1_test3, 6);
        let day06_part1_test4 = aoc2022::day06::part01("./test_puzzle_input/day06_test4.txt");
        assert_eq!(day06_part1_test4, 10);
        let day06_part1_test5 = aoc2022::day06::part01("./test_puzzle_input/day06_test5.txt");
        assert_eq!(day06_part1_test5, 11);
        let day06_part2_test1 = aoc2022::day06::part02("./test_puzzle_input/day06_test.txt");
        assert_eq!(day06_part2_test1, 19);
        let day06_part2_test2 = aoc2022::day06::part02("./test_puzzle_input/day06_test2.txt");
        assert_eq!(day06_part2_test2, 23);
        let day06_part2_test3 = aoc2022::day06::part02("./test_puzzle_input/day06_test3.txt");
        assert_eq!(day06_part2_test3, 23);
        let day06_part2_test4 = aoc2022::day06::part02("./test_puzzle_input/day06_test4.txt");
        assert_eq!(day06_part2_test4, 29);
        let day06_part2_test5 = aoc2022::day06::part02("./test_puzzle_input/day06_test5.txt");
        assert_eq!(day06_part2_test5, 26);

        let day07_part1 = aoc2022::day07::part01("./test_puzzle_input/day07_test.txt");
        assert_eq!(day07_part1, 95437);
    }
}

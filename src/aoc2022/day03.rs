pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let both_compartments = puzzle_input
        .lines()
        .map(|l| {
            let count = l.len();
            let (first_compartment, second_compartment) = l.split_at(count / 2);

            let second_compartment_types: std::collections::HashSet<char> =
                second_compartment.chars().collect();

            first_compartment
                .chars()
                .find(|c| second_compartment_types.contains(c))
        })
        .filter_map(|x| x);

    let priorities = both_compartments.map(|c| match c {
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => unreachable!(),
    });

    priorities.sum()
}

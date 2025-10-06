pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let calories_list = puzzle_input.lines().map(|l| l.parse::<u64>());
    let mut max_calories = 0;
    let mut calories_sum = 0;
    for calories in calories_list {
        if let Ok(calories) = calories {
            calories_sum += calories;
        } else {
            if calories_sum > max_calories {
                max_calories = calories_sum
            };
            calories_sum = 0;
        }
    }
    if calories_sum > max_calories {
        max_calories = calories_sum
    };
    max_calories.try_into().unwrap()
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let calories_list = puzzle_input.lines().map(|l| l.parse::<u64>());
    let mut elf_calories = vec![];
    let mut calories_sum = 0;
    for calories in calories_list {
        if let Ok(calories) = calories {
            calories_sum += calories;
        } else {
            elf_calories.push(calories_sum);
            calories_sum = 0;
        }
    }
    elf_calories.push(calories_sum);
    elf_calories.sort_unstable();

    (elf_calories[elf_calories.len() - 1]
        + elf_calories[elf_calories.len() - 2]
        + elf_calories[elf_calories.len() - 3])
        .try_into()
        .unwrap()
}

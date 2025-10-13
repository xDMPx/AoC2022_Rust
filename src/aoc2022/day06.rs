pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut datastream = puzzle_input.lines().next().unwrap().chars();

    let mut last_chars = [
        datastream.next().unwrap(),
        datastream.next().unwrap(),
        datastream.next().unwrap(),
    ];
    let mut processed = 3;
    for c in datastream {
        processed += 1;
        if !last_chars.contains(&c) {
            let unique_chars = std::collections::HashSet::from(last_chars);
            if unique_chars.len() == 3 {
                break;
            }
        }
        last_chars[0] = last_chars[1];
        last_chars[1] = last_chars[2];
        last_chars[2] = c;
    }

    processed
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut datastream = puzzle_input.lines().next().unwrap().chars();

    let mut last_chars = [' '; 13];
    let mut processed = 13;
    for i in 0..13 {
        last_chars[i] = datastream.next().unwrap();
    }
    for c in datastream {
        processed += 1;
        if !last_chars.contains(&c) {
            let unique_chars = std::collections::HashSet::from(last_chars);
            if unique_chars.len() == 13 {
                break;
            }
        }
        for i in 1..13 {
            last_chars[i - 1] = last_chars[i];
        }
        last_chars[12] = c;
    }

    processed
}

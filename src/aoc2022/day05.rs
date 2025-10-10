#[derive(Debug)]
struct RProcedure {
    to_move: usize,
    from: usize,
    to: usize,
}

pub fn part01(file_path: &str) -> String {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut puzzle_input = puzzle_input.lines();

    let mut stacks = vec![];
    while let Some(line) = puzzle_input.next() {
        if !line.trim().starts_with('[') {
            break;
        }
        let crates_positioning = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .map(|(i, c)| (((i - 1) / 4) + 1, c));

        for (stack_num, c) in crates_positioning {
            while stack_num >= stacks.len() {
                stacks.push(vec![]);
            }
            stacks[stack_num].push(c);
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    puzzle_input.next();

    let procedures = puzzle_input.map(|l| {
        let (to_move, fromto) = l.split_once('f').unwrap();
        let to_move = to_move
            .strip_prefix("move")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let (from, to) = fromto.split_once("to").unwrap();
        let from = from
            .trim()
            .strip_prefix("rom")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let to = to.trim().parse().unwrap();

        RProcedure { to_move, from, to }
    });
    for mut procedure in procedures {
        while procedure.to_move != 0 {
            let c = stacks[procedure.from].pop().unwrap();
            stacks[procedure.to].push(c);
            procedure.to_move -= 1;
        }
    }

    let message: String = stacks.iter().filter_map(|s| s.iter().last()).collect();

    message
}

pub fn part02(file_path: &str) -> String {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut puzzle_input = puzzle_input.lines();

    let mut stacks = vec![];
    while let Some(line) = puzzle_input.next() {
        if !line.trim().starts_with('[') {
            break;
        }
        let crates_positioning = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .map(|(i, c)| (((i - 1) / 4) + 1, c));

        for (stack_num, c) in crates_positioning {
            while stack_num >= stacks.len() {
                stacks.push(vec![]);
            }
            stacks[stack_num].push(c);
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    puzzle_input.next();

    let procedures = puzzle_input.map(|l| {
        let (to_move, fromto) = l.split_once('f').unwrap();
        let to_move = to_move
            .strip_prefix("move")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let (from, to) = fromto.split_once("to").unwrap();
        let from = from
            .trim()
            .strip_prefix("rom")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let to = to.trim().parse().unwrap();

        RProcedure { to_move, from, to }
    });
    for mut procedure in procedures {
        let mut tmp = vec![];
        while procedure.to_move != 0 {
            let c = stacks[procedure.from].pop().unwrap();
            tmp.push(c);
            procedure.to_move -= 1;
        }
        while let Some(c) = tmp.pop() {
            stacks[procedure.to].push(c);
        }
    }

    let message: String = stacks.iter().filter_map(|s| s.iter().last()).collect();

    message
}

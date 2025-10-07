// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let guide = puzzle_input.lines().map(|l| {
        let o = l.chars().next().unwrap();
        let r = l.chars().last().unwrap();
        (move_to_shape(o), move_to_shape(r))
    });

    let scores = guide.map(|(o, r)| {
        let outcome = match (o, r) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
        };
        let total = outcome
            + match r {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };

        total
    });

    scores.sum()
}

fn move_to_shape(c: char) -> Shape {
    match c {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => unreachable!(),
    }
}

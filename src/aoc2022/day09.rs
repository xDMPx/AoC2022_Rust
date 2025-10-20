#[derive(Debug)]
enum Motion {
    UP(usize),
    DOWN(usize),
    RIGHT(usize),
    LEFT(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let motions = puzzle_input.lines().map(|l| {
        let (direction, steps) = l.split_once(' ').unwrap();
        match direction {
            "U" => Motion::UP(steps.parse().unwrap()),
            "D" => Motion::DOWN(steps.parse().unwrap()),
            "R" => Motion::RIGHT(steps.parse().unwrap()),
            "L" => Motion::LEFT(steps.parse().unwrap()),
            _ => unreachable!(),
        }
    });

    let mut head_pos = Pos { x: 0, y: 0 };
    let mut tail_pos = Pos { x: 0, y: 0 };
    let mut visited_pos = std::collections::HashSet::new();
    for motion in motions {
        match motion {
            Motion::UP(y) => {
                for _ in 0..y {
                    head_pos.y += 1;
                    move_tail(&mut tail_pos, head_pos);
                    visited_pos.insert(tail_pos.clone());
                }
            }
            Motion::DOWN(y) => {
                for _ in 0..y {
                    head_pos.y -= 1;
                    move_tail(&mut tail_pos, head_pos);
                    visited_pos.insert(tail_pos.clone());
                }
            }
            Motion::RIGHT(x) => {
                for _ in 0..x {
                    head_pos.x += 1;
                    move_tail(&mut tail_pos, head_pos);
                    visited_pos.insert(tail_pos.clone());
                }
            }
            Motion::LEFT(x) => {
                for _ in 0..x {
                    head_pos.x -= 1;
                    move_tail(&mut tail_pos, head_pos);
                    visited_pos.insert(tail_pos.clone());
                }
            }
        }
    }

    visited_pos.len()
}

fn move_tail(tail_pos: &mut Pos, head_pos: Pos) {
    let dx = head_pos.x - tail_pos.x;
    let dy = head_pos.y - tail_pos.y;

    if dx.abs() > 1 || dy.abs() > 1 {
        tail_pos.x += dx.clamp(-1, 1);
        tail_pos.y += dy.clamp(-1, 1);
    }
}

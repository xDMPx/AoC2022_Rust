pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<usize>> = puzzle_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    let mut visible_trees = 0;
    for y in 1..max_y {
        for x in 1..max_x {
            //left
            let mut visible = *(map[y].iter().take(x).max().unwrap()) < map[y][x];
            //right
            visible |= *(map[y].iter().skip(x + 1).max().unwrap()) < map[y][x];
            //up
            visible |= map.iter().map(|r| r[x]).take(y).max().unwrap() < map[y][x];
            //down
            visible |= map.iter().map(|r| r[x]).skip(y + 1).max().unwrap() < map[y][x];

            if visible {
                visible_trees += 1;
            }
        }
    }

    visible_trees + 2 * max_y + 2 * max_x
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<usize>> = puzzle_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    let mut max_scenic_score = 0;
    for y in 1..max_y {
        for x in 1..max_x {
            //up
            let up_visible_trees =
                count_visible_trees(map.iter().map(|r| r[x]).take(y).rev(), map[y][x]);
            //left
            let left_visible_trees =
                count_visible_trees(map[y].iter().take(x).rev().copied(), map[y][x]);
            //right
            let right_visible_tress =
                count_visible_trees(map[y].iter().skip(x + 1).copied(), map[y][x]);
            //down
            let down_visible_tress =
                count_visible_trees(map.iter().map(|r| r[x]).skip(y + 1), map[y][x]);

            let scenic_score =
                up_visible_trees * left_visible_trees * right_visible_tress * down_visible_tress;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

fn count_visible_trees<I>(iter: I, height: usize) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut count = 0;
    for v in iter {
        count += 1;
        if v >= height {
            break;
        }
    }
    count
}

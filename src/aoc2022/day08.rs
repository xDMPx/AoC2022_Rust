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

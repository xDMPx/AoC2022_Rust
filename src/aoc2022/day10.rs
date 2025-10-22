pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();

    let mut x = 1;
    let mut cycle = 1;

    let mut interesting_cycles_vals = [0; 6];
    let interesting_cycles: [u64; 6] = [20, 60, 100, 140, 180, 220];
    let mut i = 0;

    for instruction in puzzle_input.lines() {
        let mut value = 0;
        let start_cycle = cycle;
        if instruction.starts_with("noop") {
            cycle += 0;
            value += 0;
        } else {
            let v: i64 = instruction.split_once(' ').unwrap().1.parse().unwrap();
            value += v;
            cycle += 1;
        }
        if start_cycle <= interesting_cycles[i] && cycle >= interesting_cycles[i] {
            interesting_cycles_vals[i] = x;
            i += 1;
        }
        if i > 5 {
            break;
        }
        cycle += 1;
        x += value;
    }

    interesting_cycles_vals
        .iter()
        .zip(interesting_cycles)
        .map(|(&v, c)| (v as u64) * c)
        .sum::<u64>()
        .try_into()
        .unwrap()
}

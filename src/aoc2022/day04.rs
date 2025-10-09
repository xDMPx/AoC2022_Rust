#[derive(Clone, Copy)]
struct Section {
    start: usize,
    end: usize,
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let assignments = puzzle_input.lines().map(|l| {
        let (l, r) = l.split_once(',').unwrap();
        let (l_s, l_e) = l.split_once('-').unwrap();
        let (r_s, r_e) = r.split_once('-').unwrap();

        let l_section = Section {
            start: l_s.parse().unwrap(),
            end: l_e.parse().unwrap(),
        };
        let r_section = Section {
            start: r_s.parse().unwrap(),
            end: r_e.parse().unwrap(),
        };

        (l_section, r_section)
    });

    let fully_contain_pairs = assignments
        .map(|(l, r)| fully_contains(l, r) || fully_contains(r, l))
        .filter(|&contains| contains);

    fully_contain_pairs.count()
}

fn fully_contains(x: Section, y: Section) -> bool {
    y.start >= x.start && y.end <= x.end
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let assignments = puzzle_input.lines().map(|l| {
        let (l, r) = l.split_once(',').unwrap();
        let (l_s, l_e) = l.split_once('-').unwrap();
        let (r_s, r_e) = r.split_once('-').unwrap();

        let l_section = Section {
            start: l_s.parse().unwrap(),
            end: l_e.parse().unwrap(),
        };
        let r_section = Section {
            start: r_s.parse().unwrap(),
            end: r_e.parse().unwrap(),
        };

        (l_section, r_section)
    });

    let overlaping_pairs = assignments
        .map(|(l, r)| overlap_at_all(l, r) || overlap_at_all(r, l))
        .filter(|&contains| contains);

    overlaping_pairs.count()
}

fn overlap_at_all(x: Section, y: Section) -> bool {
    y.start >= x.start && y.start <= x.end
}

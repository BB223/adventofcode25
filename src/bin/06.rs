use itertools::izip;

#[derive(Debug, Clone)]
enum Operations {
    Add,
    Mul,
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut nums: Vec<Vec<u64>> = vec![vec![]; grid[0].len()];
    let mut ops: Vec<Operations> = vec![Operations::Mul; grid[0].len()];

    for line in grid {
        for (i, element) in line.iter().enumerate() {
            if let Ok(num) = element.parse::<u64>() {
                nums[i].push(num);
            } else if *element == "+" {
                ops[i] = Operations::Add;
            }
        }
    }

    let solution = nums
        .iter()
        .enumerate()
        .filter_map(|(i, num)| match ops[i] {
            Operations::Add => num.iter().copied().reduce(|acc, n| acc + n),
            Operations::Mul => num.iter().copied().reduce(|acc, n| acc * n),
        })
        .sum();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let msn = lines.next().unwrap().chars();
    let mmn = lines.next().unwrap().chars();
    let lmn = lines.next().unwrap().chars();
    let lsn = lines.next().unwrap().chars();
    let ops = lines.next().unwrap().split_whitespace();
    let column = izip!(msn, mmn, lmn, lsn);

    let nums: Vec<u64> = column
        .filter_map(|(msn, mmn, lmn, lsn)| {
            format!("{}{}{}{}", msn, mmn, lmn, lsn)
                .replace("    ", "0")
                .trim()
                .parse()
                .ok()
        })
        .collect();
    let solution = nums
        .split(|n| *n == 0)
        .zip(ops)
        .filter_map(|(p, op)| {
            if op == "+" {
                p.iter().copied().reduce(|acc, n| acc + n)
            } else {
                p.iter().copied().reduce(|acc, n| acc * n)
            }
        })
        .sum();

    Some(solution)
}

advent_of_code_25::advent_of_code!(6, Some(4277556), Some(3263827));

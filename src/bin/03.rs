pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut nums = vec![];
    for bank in banks {
        let mut collect = vec![];
        let batteries = bank.chars();
        let first = batteries.clone().take(1).next().unwrap();
        let mut max = first;
        let mut max_position = 0;
        for (position, battery) in batteries.clone().skip(1).enumerate() {
            if battery > max {
                max = battery;
                max_position = position + 1;
            }
        }
        collect.push(max);
        let mut max2 = first;
        if max_position + 1 == batteries.clone().count() {
            for (position, battery) in batteries.clone().skip(1).enumerate() {
                if battery > max2 && position + 1 != max_position {
                    max2 = battery;
                }
            }
            collect.push(max2);
            collect.reverse();
            nums.push(collect);
            continue;
        }
        let mut max2 = batteries
            .clone()
            .skip(max_position + 1)
            .take(1)
            .next()
            .unwrap();
        for battery in batteries.skip(max_position + 1) {
            if battery > max2 {
                max2 = battery;
            }
        }
        collect.push(max2);
        nums.push(collect);
    }

    let solution = nums
        .iter()
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|s| s.parse::<u64>().unwrap())
        .sum();
    Some(solution)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut nums = vec![];
    for bank in banks {
        let d = max_fn(bank, 12)?;
        nums.push(d.parse().unwrap());
    }
    Some(nums.iter().sum())
}

fn max_fn(batteries: &str, remaining: i32) -> Option<String> {
    if remaining <= 0 {
        return None;
    }
    let mut max = '/';
    let mut max_position = 0;
    for (position, battery) in batteries.chars().enumerate() {
        if battery > max {
            max = battery;
            max_position = position;
        }
    }
    while batteries.chars().skip(max_position).count() < remaining as usize {
        max = '/';
        let bat = batteries.chars().take(max_position);
        max_position = 0;
        for (position, battery) in bat.enumerate() {
            if battery > max {
                max = battery;
                max_position = position;
            }
        }
    }
    let next = max_fn(&batteries[max_position + 1..], remaining - 1);
    match next {
        Some(x) => Some(format!("{max}{x}")),
        None => Some(format!("{max}")),
    }
}

advent_of_code_25::advent_of_code!(3, Some(357), Some(3121910778619));

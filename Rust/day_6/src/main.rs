fn main() {
    let input1 = include_str!("input1.txt");
    let sol1 = first_part(input1);
    dbg!(sol1);

    let input2 = include_str!("input1.txt");
    let sol2 = second_part(input2);
    dbg!(sol2);

}

fn first_part(input: &str) -> usize {
    let lines = input.split('\n');
    
    get_score(lines)
}

fn second_part(input: &str) -> usize {
    let lines = input.split('\n');
    
    get_score_big(lines)
}


fn get_score<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    let mut res:usize = 1;

    for line in lines {
        if line.contains("Time") {
            times = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            continue;
        }
        
        if line.contains("Distance") {
            distances = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            continue;
        }
    }

    for (time, distance) in times.into_iter().zip(distances) {
        let mut times_win = 0;
        for hold_b in 0..=time {
            let act_dist = hold_b * (time - hold_b);
            if act_dist > distance {
                times_win += 1;
            }
        }
        res *= times_win;
    }
    
    res
}


fn get_score_big<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut time:usize = 0;
    let mut distance:usize = 0;
    let mut res:usize = 0;

    for line in lines {
        if line.contains("Time") {
            time = line
                .split("Time:")
                .collect::<String>()
                .replace(" ", "")
                .parse().unwrap();
            continue;
        }
        
        if line.contains("Distance") {
            distance = line
                .split("Distance:")
                .collect::<String>()
                .replace(" ", "")
                .parse().unwrap();
            continue;
        }
    }

    for hold_b in 0..=time {
        let act_dist = hold_b * (time - hold_b);
        if act_dist > distance {
            res += 1;
        }
    }
    
    res

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let sol = first_part(input);
        assert_eq!(sol, 288);
    }

    #[test]
    fn test_second_part() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let sol = second_part(input);
        assert_eq!(sol, 71503);

    }
}

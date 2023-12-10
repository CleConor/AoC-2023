use std::usize;
use std::collections::{HashSet, HashMap};

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

    get_card_points(lines)
}

fn second_part(input: &str) -> usize {
    let lines = input.split('\n');

    get_sum_instances(lines)
}

fn get_card_points<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .flat_map(|l| l.split(':'))
        .flat_map(|all_numbers| all_numbers.split('|'))
        .map(|parts| {
            parts.split_whitespace().filter_map(|el| el.parse::<usize>().ok()).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(3)
        .filter(|chunk| chunk.len() > 1)
        .flat_map(|chunk| {
            let right_values: HashSet<_> = chunk[1].iter().cloned().collect();
            let c = chunk[2].iter().filter(move |&&x| right_values.contains(&x)).count();
            if c == 0 {
                vec![]
            } else {
                (1..c).fold(vec![1], |mut acc, _| {
                    let last_value = *acc.last().unwrap();
                    acc[0] = last_value * 2;
                    acc
                })
            }
        }).sum()
}

fn get_sum_instances<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    lines
        .flat_map(|l| l.split(':'))
        .flat_map(|all_numbers| all_numbers.split('|'))
        .map(|parts| {
            parts.split_whitespace().filter_map(|el| el.parse::<usize>().ok()).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(3)
        .filter(|chunk| chunk.len() > 1)
        .fold((0, HashMap::<usize, usize>::new()), |acc, chunk| {
            let n = chunk[0].first().unwrap();
            let right_values: HashSet<_> = chunk[1].iter().cloned().collect();
            let values = chunk[2].iter().filter(move |&&x| right_values.contains(&x)).count();

            let v = acc.1.get(n).unwrap_or(&0);

            let ist = v + 1;
            let mut matches = (0..values).map(|v| (n + 1 + v, ist)).collect::<HashMap<usize, usize>>();

            acc.1.iter().for_each(|k_v| {
                matches
                    .entry(*k_v.0)
                    .and_modify(|v| *v += *k_v.1)
                    .or_insert(*k_v.1);
            });

                        
            (acc.0 + ist, matches)
        }).0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let sol = first_part(input);
        assert_eq!(sol, 13);
    }

    #[test]
    fn test_second_part() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let sol = second_part(input);
        assert_eq!(sol, 30);

    }
}

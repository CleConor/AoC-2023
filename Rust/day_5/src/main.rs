use std::cmp;

fn main() {
    let input1 = include_str!("input1.txt");
    let sol1 = first_part(input1);
    dbg!(sol1);

    let input2 = include_str!("input1.txt");
    let sol2 = second_part(input2);
    dbg!(sol2);

}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<(usize, usize, usize)>>,
}

fn first_part(input: &str) -> usize {
    let lines = input.split('\n');

    get_lowest_location(lines)
}

fn second_part(input: &str) -> usize {
    let lines = input.split('\n');
    
    get_lowest_location_range(lines)
}


fn get_lowest_location<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut res:usize = usize::MAX;
    let mut almanac = Almanac { seeds:vec![], maps:vec![vec![]; 7] };
    let mut i:usize = 0;
    
    lines
        .filter(|line| line.len() > 0)
        .for_each(|line| {
        if line.contains("seeds") {
            almanac.seeds = line.split(" ").filter_map(|s| s.parse::<usize>().ok()).collect();
        } else if line.contains("map") {
            i += 1;
        } else {
            let map = line.split(" ").filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();
            almanac.maps[i - 1].push((map[0], map[1], map[1] + map[2] - 1));
        }
    });

    for seed in almanac.seeds {
        let mut pos = seed;
        for map in &almanac.maps {
            for (dest, min, max) in map {
                if pos >= *min && pos <= *max {
                    pos = *dest + (pos - *min);
                    break;
                }
            }
        }
        res = cmp::min(pos, res);
    }

    res
}

fn get_lowest_location_range<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut res:usize = usize::MAX;
    let mut almanac = Almanac { seeds:vec![], maps:vec![vec![]; 7] };
    let mut i:usize = 0;    

    lines
        .filter(|line| line.len() > 0)
        .for_each(|line| {
        if line.contains("seeds") {
            almanac.seeds = line.split(" ").filter_map(|s| s.parse::<usize>().ok()).collect();
        } else if line.contains("map") {
            i += 1;
        } else {
            let map = line.split(" ").filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();
            almanac.maps[i - 1].push((map[0], map[1], map[1] + map[2] - 1));
        }
    });

    almanac.seeds
        .chunks(2)
        .for_each(|chunk| {
            for seed in 0..=chunk[1] {
                let mut pos = seed + chunk[0];
                for map in &almanac.maps {
                    for (dest, min, max) in map {
                        if pos >= *min && pos <= *max {
                            pos = *dest + (pos - *min);
                            break;
                        }
                    }
                }
                res = cmp::min(pos, res);
            }
        });
    
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let sol = first_part(input);
        assert_eq!(sol, 35);
    }

    #[test]
    fn test_second_part() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let sol = second_part(input);
        assert_eq!(sol, 46);

    }
}

use std::collections::HashMap;
use std::cmp;

fn main() {
    let input1 = include_str!("input1.txt");
    let sol1 = first_part(input1);
    dbg!(sol1);

    let input2 = include_str!("input1.txt");
    let sol2 = second_part(input2);
    dbg!(sol2);

}

fn first_part(input: &str) -> usize {
    let mut line = "".to_string();
    let mut sol: usize = 0;
    for c in input.chars() {
        if c == '\n' {
            sol += get_valid_game(line);
            line = "".to_string();
            continue;
        }
        line.push(c);
    }

    sol
}

fn second_part(input: &str) -> usize {
    let mut line = "".to_string();
    let mut sol: usize = 0;
    for c in input.chars() {
        if c == '\n' {
            sol += get_mult_max(line);
            line = "".to_string();
            continue;
        }
        line.push(c);
    }

    sol

}

fn get_valid_game(line: String) -> usize {
    let max_game_colors = HashMap::from([
        ("red", 12),
        ("blue", 14),
        ("green", 13),
    ]);

    let mut actual_game = HashMap::new();

    let n_game = line.clone().split(":").next().unwrap().split(" ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();
    
    let sets = line.split(":").collect::<Vec<_>>()[1].split(";");

    let _ = sets.map(|cube| {
        let _ = cube.split(",").map(|color| {
            let n_and_c: Vec<&str> = color.splitn(3, " ").collect();
            actual_game.insert(n_and_c[2], cmp::max(*actual_game.get(n_and_c[2]).unwrap_or(&0), n_and_c[1].parse::<usize>().unwrap()));
        }).collect::<Vec<_>>();
    }).collect::<Vec<_>>();
    
    for k in max_game_colors.keys() {
        if actual_game.get(k).unwrap() > max_game_colors.get(k).unwrap() {
            return 0;
        }
    }

    n_game
}

fn get_mult_max(line: String) -> usize {
    let mut actual_game = HashMap::new();

    let sets = line.split(":").collect::<Vec<_>>()[1].split(";");

    let _ = sets.map(|cube| {
        let _ = cube.split(",").map(|color| {
            let n_and_c: Vec<&str> = color.splitn(3, " ").collect();
            actual_game.insert(n_and_c[2], cmp::max(*actual_game.get(n_and_c[2]).unwrap_or(&0), n_and_c[1].parse::<usize>().unwrap()));
        }).collect::<Vec<_>>();
    }).collect::<Vec<_>>();
    
    let res = actual_game.values().cloned().reduce(|acc, e| acc * e).unwrap();

    res
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let sol = first_part(input);
        assert_eq!(sol, 8);
    }

    #[test]
    fn test_second_part() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let sol = second_part(input);
        assert_eq!(sol, 2286);

    }
}

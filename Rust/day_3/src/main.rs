use std::{collections::HashSet, i32};

fn main() {
    let input1 = include_str!("input1.txt");
    let sol1 = first_part(input1);
    dbg!(sol1);

    /*let input2 = include_str!("input1.txt");
    let sol2 = second_part(input2);
    dbg!(sol2);*/

}

fn first_part(input: &str) -> usize {
    let lines = input.split('\n');

    sum_valid_number(lines)
}

fn sum_valid_number<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut valid_number_i = HashSet::new();
    let mut sol:usize = 0;
    let mut number_str = "".to_string();
    let mut is_valid = false;
    let copy_line_string = lines.map(|s| s.to_string()).collect::<Vec<_>>();

    for (row, line) in copy_line_string.iter().enumerate() {
        for (c_i, c) in line.char_indices() {
            if !c.is_numeric() && c != '.' {
                for i in -1..2 {
                    for j in -1..2 {
                        let a = row as i32 + i;
                        let b = (c_i as i32) + j;

                        if b == c_i as i32 && row as i32 == a {
                            continue;
                        }
                        valid_number_i.insert((a, b));
                    }
                }
            }
        }
    }
    
    for (row, line) in copy_line_string.iter().enumerate() {
        for (c_i, c) in line.char_indices() {
            if c.is_numeric() {
                number_str.push(c);
                if valid_number_i.get(&(row as i32, c_i as i32)).is_some() {
                    is_valid = true;
                }
            }else {
                if is_valid {
                    //dbg!(&number_str);
                    sol += number_str.parse::<usize>().unwrap();
                    is_valid = false;
                }
                number_str = "".to_string();
            }
        }
        if is_valid {
            sol += number_str.parse::<usize>().unwrap();
            is_valid = false;
            number_str = "".to_string();
        }
    }

    sol
}

/*fn first_part(input: &str) -> usize {
    let mut line = "".to_string();
    let mut sol: usize = 0;
    for c in input.chars() {
        if c == '\n' {
            //sol += get_valid_game(line);
            line = "".to_string();
            continue;
        }
        line.push(c);
    }

    sol
}*/

/*fn second_part(input: &str) -> usize {
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

}*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let sol = first_part(input);
        assert_eq!(sol, 436);
    }

    /*#[test]
    fn test_second_part() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let sol = second_part(input);
        assert_eq!(sol, 2286);

    }*/
}

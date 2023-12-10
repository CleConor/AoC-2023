use std::collections::HashMap;

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
            sol += concat_first_and_last(line);
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
            sol += concat_first_and_last_str(line);
            line = "".to_string();
            continue;
        }
        line.push(c);
    }

    sol

}

fn concat_first_and_last(line: String) -> usize {
    let mut number = "".to_string(); 
   
    number.push( line.chars().filter(|x| x.is_numeric()).next().unwrap());
    number.push( line.chars().rev().filter(|x| x.is_numeric()).next().unwrap());

    number.parse::<usize>().unwrap()
}

fn concat_first_and_last_str(mut line: String) -> usize {
    let map = HashMap::from([
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);

    for (k, v) in map.iter() {
        line = line.replace(k, v)
    }

    concat_first_and_last(line)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let sol = first_part(input);
        assert_eq!(sol, 142);
    }

    #[test]
    fn test_second_part() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let sol = second_part(input);
        assert_eq!(sol, 281);

    }
}

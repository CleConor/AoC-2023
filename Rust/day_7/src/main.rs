fn main() {
<<<<<<< HEAD
    let input1 = include_str!("input1.txt");
    let sol1 = first_part(input1);
    dbg!(sol1);

    /*let input2 = include_str!("input1.txt");
    let sol2 = second_part(input2);
    dbg!(sol2);*/

}

fn first_part(input: &str) -> usize {
    let lines = input.split('\n');
    
    get_total_wining(lines)
}

/*fn second_part(input: &str) -> usize {
    let lines = input.split('\n');
    
}*/


fn get_total_wining<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "";
        let sol = first_part(input);
        assert_eq!(sol, 35);
    }

    /*#[test]
    fn test_second_part() {
        let input = "";
        let sol = second_part(input);
        assert_eq!(sol, 46);
    }*/
=======
    println!("Hello, world!");
>>>>>>> b2ef8a0fb67b07dd91dccb45e6df4732822e9c72
}

fn main() {

    let input_file = include_str!("../../input1.txt");
    let output = part1(input_file);

    println!("{:?}", output);

    dbg!(output.unwrap().unwrap());
    
}

fn part1(input: &str) -> Result<Option<u32>, String> {

    let output = input
        .lines()
        .map(|line| {
            
            let mut iter = line
                .chars()
                .filter_map(|character| { 
                    character.to_digit(10)
                });

            let first = iter
                .next()
                .expect("Should be a number");
            
            let last  = iter
                .last();

            match last {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }.parse::<u32>().expect("should be a valid number")

        })
        .sum::<u32>();

    Ok(Some(output))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        
        let result = part1(input);

        assert_eq!(result, Ok(Some(142)));
    }
}
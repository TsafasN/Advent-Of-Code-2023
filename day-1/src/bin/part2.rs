fn main() {

    let input_file = include_str!("../../input1.txt");
    let output = part1(input_file);

    println!("{:?}", output);

    dbg!(output.unwrap().unwrap());
    
}

fn part1(input: &str) -> Result<Option<u32>, String> {

    let output = input
        .lines()
        .map(process_line)
        .sum::<u32>();

    Ok(Some(output))

}

fn process_line(line: &str) -> u32{
    let line = line
    .replace("one", "1")
    .replace("two", "2")
    .replace("three", "3")
    .replace("four", "4")
    .replace("five", "5")
    .replace("six", "6")
    .replace("seven", "7")
    .replace("eight", "8")
    .replace("nine", "9")
    .replace("zero", "0");
    
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

}



#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        
        let result = part1(input);

        assert_eq!(Ok(Some(281)), result);
    }
}
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

    let mut iter = (0..line.len()).filter_map(
        |index| {
            let reduced_line = &line[index..];
            let result = 
            if reduced_line.starts_with("one"){
                '1'
            } else if reduced_line.starts_with("two"){
                '2'
            } else if reduced_line.starts_with("three"){
                '3'
            } else if reduced_line.starts_with("four"){
                '4'
            } else if reduced_line.starts_with("five"){
                '5'
            } else if reduced_line.starts_with("six"){
                '6'
            } else if reduced_line.starts_with("seven"){
                '7'
            } else if reduced_line.starts_with("eight"){
                '8'
            } else if reduced_line.starts_with("nine"){
                '9'
            } else if reduced_line.starts_with("zero"){
                '0'
            } else {
                reduced_line.chars().next().unwrap()
            };
            
            result.to_digit(10)
        }
    );
    
    let first = iter
        .next()
        .expect("Should be a number");
    
    let last  = iter
        .last();

    match last {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")

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
    // this test is from the real input
    // it tests two overlapping numbers
    // where the second number should succeed
    #[case("five75pvngkvx9nnlttwotwonev", 51)]
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
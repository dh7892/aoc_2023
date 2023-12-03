use regex::Regex;

advent_of_code::solution!(1);

fn line_to_number(line: &str) -> u32 {
    let a = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .expect("Can't find first digit on line {}");
    let b = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .expect("Can't find last digit on line {}");
    format!("{}{}", a, b).parse::<u32>().expect("Invalid input")
}

fn first_digit(line: &str) -> u32 {
    // Find the first digit or word that spells a digit
    // First, we find the first match of any of the following:
    // 0,1,2,3,4,5,6,7,8,9,one,two,three,four,five,six,seven,eight,nine
    let re = Regex::new(r"(?i)(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let first_match = re.find(line).unwrap();
    match first_match.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => first_match.as_str().parse::<u32>().ok().unwrap(),
    }
}

fn last_digit(line: &str) -> u32 {
    // Find the last digit or word that spells a digit
    // First, we find the first match of any of the following:
    // 0,1,2,3,4,5,6,7,8,9,one,two,three,four,five,six,seven,eight,nine
    let re = Regex::new(r"(?i)(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let reversed_line = line.chars().rev().collect::<String>();
    let last_match = re.find(&reversed_line).unwrap();
    match last_match.as_str() {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => last_match.as_str().parse::<u32>().ok().unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(line_to_number).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .inspect(|l| println!("{}", l))
            .map(|l| {
                let first_digit = first_digit(l);
                let last_digit = last_digit(l);
                format!("{}{}", first_digit, last_digit)
                    .parse::<u32>()
                    .unwrap()
            })
            .inspect(|n| println!("{}", n))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_custom_file(
            "examples",
            "day01_part_a.txt",
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_first_and_last() {
        let line = "4eightfivefivetwooneightvhr";
        let expected = "48";
        let result = format!("{}{}", first_digit(line), last_digit(line));
        assert_eq!(result, expected);
    }
}

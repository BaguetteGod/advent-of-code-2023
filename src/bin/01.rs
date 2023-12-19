advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let digits = line
            .chars()
            .filter_map(|char| char.to_digit(10))
            .collect::<Vec<u32>>();

        let first = digits.first().unwrap().to_owned();
        let last = digits.last().unwrap().to_owned();

        sum += first * 10 + last;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;
    for line in input.lines() {
        let mut digits: Vec<u32> = Vec::new();
        for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                digits.push(char.to_digit(10).unwrap())
            }
            for (index_num, number) in NUMBERS.iter().enumerate() {
                if line[index..].starts_with(number) {
                    digits.push(index_num as u32 + 1)
                }
            }
        }
        let first = digits.first().unwrap().to_owned();
        let last = digits.last().unwrap().to_owned();

        sum += first * 10 + last;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    'game: for (idx, line) in input.lines().enumerate() {
        let id = idx + 1;
        let (_, draws) = line.split_once(": ").unwrap();
        for draw in draws.split("; ") {
            for pair in draw.split(", ") {
                let (num, color) = pair.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                let possible = match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    _ => panic!("This shouldn't be possible"),
                };
                if !possible {
                    continue 'game;
                }
            }
        }

        sum += id;
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        let (_, draws) = line.split_once(": ").unwrap();

        for draw in draws.split("; ") {
            for pair in draw.split(", ") {
                let (num, color) = pair.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => min_red = min_red.max(num),
                    "green" => min_green = min_green.max(num),
                    "blue" => min_blue = min_blue.max(num),
                    _ => panic!("This shouldn't be possible"),
                };
            }
        }

        sum += min_red * min_green * min_blue;
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

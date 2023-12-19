use itertools::Itertools;
use regex::{Match, Regex};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    id: u32,
    line: u32,
    value: u32,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(line: {}, value: {})", self.line, self.value)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let special_re = Regex::new(r"[^.0-9\r\n]").unwrap();

    let lines = input.lines();
    let mut parts: Vec<Part> = Vec::new();
    for (current, (line, next)) in lines.tuple_windows().enumerate() {
        let has_more_lines = !next.is_empty();
        let special_current: Vec<Match> = special_re.find_iter(line).collect();
        let numbers_current: Vec<Match> = number_re.find_iter(line).collect();
        let special_next: Vec<Match> = special_re.find_iter(next).collect();
        let numbers_next: Vec<Match> = number_re.find_iter(next).collect();

        'number: for (index, number) in numbers_current.iter().enumerate() {
            for special in special_current.iter() {
                let is_adjacent =
                    number.start() == special.end() || number.end() == special.start();

                if is_adjacent {
                    let part = Part {
                        id: index as u32,
                        line: current as u32,
                        value: number.as_str().parse::<u32>().unwrap(),
                    };
                    parts.push(part);
                    continue 'number;
                }
            }
            if has_more_lines {
                for special in special_next.iter() {
                    let number_range = number.start()..number.end();
                    let is_adjacent =
                        number.start() == special.end() || number.end() == special.start();

                    let has_overlap = number_range.contains(&special.start());

                    if is_adjacent || has_overlap {
                        let part = Part {
                            id: index as u32,
                            line: current as u32,
                            value: number.as_str().parse::<u32>().unwrap(),
                        };
                        parts.push(part);
                        continue 'number;
                    }
                }
            }
        }
        if has_more_lines {
            'number: for (index, number) in numbers_next.iter().enumerate() {
                for special in special_current.iter() {
                    let number_range = number.start()..number.end();
                    let is_adjacent =
                        number.start() == special.end() || number.end() == special.start();

                    let has_overlap = number_range.contains(&special.start());
                    if is_adjacent || has_overlap {
                        let part = Part {
                            id: index as u32,
                            line: current as u32 + 1,
                            value: number.as_str().parse::<u32>().unwrap(),
                        };
                        parts.push(part);
                        continue 'number;
                    }
                }
            }
        }
    }
    parts.sort_by_key(|part| (part.line, part.value));
    parts.dedup_by(|a, b| a.line == b.line && a.id == b.id);

    Some(parts.iter().map(|part| part.value).sum())
}

fn get_gear(gears: &[Match], numbers_one: &[Match], numbers_two: &[Match], current: usize) -> u32 {
    let mut gear_one: u32 = 0;
    let mut gear_two: u32 = 0;
    for gear in gears.iter() {
        for number in numbers_one.iter() {
            let number_range = number.start()..number.end();
            let is_adjacent = number.start() == gear.end() || number.end() == gear.start();
            let has_overlap = number_range.contains(&gear.start());

            if is_adjacent || has_overlap {
                gear_one = number.as_str().parse().unwrap();
            }
        }
        for number in numbers_two.iter() {
            let number_range = number.start()..number.end();
            let is_adjacent = number.start() == gear.end() || number.end() == gear.start();
            let has_overlap = number_range.contains(&gear.start());

            if is_adjacent || has_overlap {
                gear_two = number.as_str().parse().unwrap();
            }
        }

        if gear_one > 0 && gear_two > 0 {
            println!(
                "line: {}: {:?}, {:?} {:?}",
                current + 1,
                gear,
                gear_one,
                gear_two
            );
        }
    }
    gear_one * gear_two
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let gear_re = Regex::new(r"\*").unwrap();
    let gear_same_line = Regex::new(r"[0-9]+\*[0-9]+").unwrap();

    let lines = input.lines();
    let mut gear_total: u32 = 0;
    for (current, (line_current, line_next, line_third)) in
        lines.tuple_windows::<(_, _, _)>().enumerate()
    {
        let numbers_current: Vec<Match> = number_re.find_iter(line_current).collect();
        let numbers_next: Vec<Match> = number_re.find_iter(line_next).collect();
        let gears_current_line: Vec<Match> = gear_re.find_iter(line_current).collect();
        let gears_middle_line: Vec<Match> = gear_re.find_iter(line_next).collect();

        gear_total += gear_same_line
            .find_iter(line_current)
            .map(|cap| {
                let (gear_one, gear_two) = cap.as_str().split_once('*').unwrap();
                gear_one.parse::<u32>().unwrap() * gear_two.parse::<u32>().unwrap()
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();

        gear_total += get_gear(
            &gears_current_line,
            &numbers_current,
            &numbers_next,
            current,
        );
        gear_total += get_gear(&gears_middle_line, &numbers_current, &numbers_next, current);

        let line_three_empty = line_third.is_empty();
        if !line_three_empty && !gears_middle_line.is_empty() {
            let numbers_third: Vec<Match> = number_re.find_iter(line_third).collect();
            if numbers_third.is_empty() || numbers_current.is_empty() {
                continue;
            }
            gear_total += get_gear(
                &gears_middle_line,
                &numbers_current,
                &numbers_third,
                current,
            );
        }
    }
    println!("{:?}", gear_total);
    Some(gear_total)
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

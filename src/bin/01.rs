advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let calibration_text: String = String::from(line);
        let digits: Vec<u32> = calibration_text
            .chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let num = digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0);

        sum += num as u64;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    let list = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let first_occ_digits = list.map(|x| (line.find(x).map(|x| x as i32), x));
        let last_occ_digits = list.map(|x| (line.rfind(x).map(|x| x as i32), x));

        let mut first_digit = first_occ_digits
            .iter()
            .min_by_key(|&(optional_index, _)| optional_index.unwrap_or(i32::MAX));

        if first_occ_digits
            .iter()
            .all(|&(usize_option, _)| usize_option.is_none())
        {
            first_digit = None;
        }

        let mut last_digit = last_occ_digits
            .iter()
            .max_by_key(|&(optional_index, _)| optional_index.unwrap_or(i32::MIN));

        if last_occ_digits
            .iter()
            .all(|&(usize_option, _)| usize_option.is_none())
        {
            last_digit = None;
        }

        let n1 = translate_to_digit(first_digit.unwrap_or(&(Some(0), "0")).1);
        let n2 = translate_to_digit(last_digit.unwrap_or(&(Some(0), "0")).1);

        let num = n1 * 10 + n2;

        sum += num as u64;
    }

    Some(sum)
}

fn translate_to_digit(input: &str) -> i32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        &_ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}

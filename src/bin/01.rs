advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let calibration_text: String = String::from(line);
        let mut num: u32 = 0;
        for char1 in calibration_text.chars() {
            if char1.is_ascii_digit() {
                num += char1.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for char1 in calibration_text.chars().rev() {
            if char1.is_ascii_digit() {
                num += char1.to_digit(10).unwrap();
                break;
            }
        }

        sum += num as u64;
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let calibration_text: String = String::from(line);
        let num: u32;
        let n1 = find_digit(&calibration_text, false) * 10;
        let n2 = find_digit(&calibration_text, true);

        num = n1 + n2;

        sum += num as u64;
    }

    return Some(sum);
}

fn find_digit(input: &str, reverse: bool) -> u32 {
    let list = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut found_digit: Option<usize> = None;
    let mut digit_found_at_index: Option<usize> = None;

    for (index, value) in list.iter().enumerate() {
        if let Some(curr_found_index) = find_digit_or_string(
            input,
            char::from_digit(index as u32, 10).unwrap(),
            value,
            reverse,
        ) {
            if !reverse {
                // Update min_index and min_value if it's the first iteration or if the current result is smaller
                if digit_found_at_index.map_or(true, |curr_min| curr_found_index < curr_min) {
                    found_digit = Some(index);
                    digit_found_at_index = Some(curr_found_index);
                }
            } else {
                // Update max_index and max_value if it's the first iteration or if the current result is larger
                if digit_found_at_index.map_or(true, |curr_max| curr_found_index > curr_max) {
                    found_digit = Some(index);
                    digit_found_at_index = Some(curr_found_index);
                }
            }
        }
    }

    return found_digit.unwrap_or(0) as u32;
}

fn find_digit_or_string(
    input: &str,
    digit: char,
    digit_string: &str,
    reverse: bool,
) -> Option<usize> {
    let str_place: Option<usize>;
    let dig_place: Option<usize>;

    if !reverse {
        str_place = input.find(digit_string);
        dig_place = input.find(digit);
    } else {
        str_place = input.rfind(digit_string);
        dig_place = input.rfind(digit);
    }

    if !reverse {
        let min = match (str_place, dig_place) {
            (Some(value1), Some(value2)) => Some(value1.min(value2)),
            (Some(value1), None) => Some(value1),
            (None, Some(value2)) => Some(value2),
            (None, None) => None,
        };

        return min;
    } else {
        let max = match (str_place, dig_place) {
            (Some(value1), Some(value2)) => Some(value1.max(value2)),
            (Some(value1), None) => Some(value1),
            (None, Some(value2)) => Some(value2),
            (None, None) => None,
        };

        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

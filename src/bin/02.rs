advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    const POSSIBLE_RED: u32 = 12;
    const POSSIBLE_BLUE: u32 = 14;
    const POSSIBLE_GREEN: u32 = 13;

    let games: Vec<Game> = input.lines().map(parse_game).collect();

    let sum_game_id: u32 = games
        .iter()
        .map(|x| (x.game_id, min_possible_counts(x.counts.clone())))
        .filter(|(_, count)| {
            count.red <= POSSIBLE_RED
                && count.blue <= POSSIBLE_BLUE
                && count.green <= POSSIBLE_GREEN
        })
        .map(|(game_id, _)| game_id)
        .sum();

    Some(sum_game_id)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(parse_game).collect();
    let sum_of_power: u32 = games
        .iter()
        .map(|x| x.counts.clone())
        .map(min_possible_counts)
        .map(|x| x.red * x.blue * x.green)
        .sum();

    Some(sum_of_power)
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    counts: Vec<Count>,
}

#[derive(Debug, Clone)]
struct Count {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_game(line: &str) -> Game {
    let parts: Vec<&str> = line.split(':').collect();
    let game_id: u32 = parts[0]
        .trim_start_matches("Game")
        .trim()
        .parse()
        .expect("Failed to parse game_id");
    let counts: Vec<Count> = parts[1].trim().split(';').map(parse_counts).collect();

    Game { game_id, counts }
}

fn parse_counts(count_str: &str) -> Count {
    let mut count = Count {
        red: 0,
        blue: 0,
        green: 0,
    };

    for color in count_str.split(',').map(|str| str.trim()) {
        let tokens: Vec<&str> = color.split_ascii_whitespace().collect();
        let value: u32 = tokens[0].parse().expect("Failed to parse count value");

        match tokens[1] {
            "red" => count.red = value,
            "blue" => count.blue = value,
            "green" => count.green = value,
            &_ => panic!("unexpected color"),
        };
    }

    count
}

fn min_possible_counts(counts: Vec<Count>) -> Count {
    Count {
        red: counts.iter().map(|c| c.red).max().unwrap_or(0),
        blue: counts.iter().map(|c| c.blue).max().unwrap_or(0),
        green: counts.iter().map(|c| c.green).max().unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

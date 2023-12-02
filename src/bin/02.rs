advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let possible_red: u32 = 12;
    let possible_blue: u32 = 14;
    let possible_green: u32 = 13;

    let games: Vec<Game> = input.lines().map(parse_game).collect();

    let sum_game_id: u32 = games
        .iter()
        .filter(|&x| is_game_valid(x, possible_red, possible_blue, possible_green))
        .map(|x| x.game_id)
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
    let game_id: u32 = parts[0].trim_start_matches("Game").trim().parse().unwrap();
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
        let value: u32 = tokens[0].parse().unwrap();

        match tokens[1] {
            "red" => count.red = value,
            "blue" => count.blue = value,
            "green" => count.green = value,
            &_ => panic!("unexpected color"),
        };
    }

    count
}

fn is_game_valid(game: &Game, max_red: u32, max_blue: u32, max_green: u32) -> bool {
    let valid_counts: Vec<&Count> = game
        .counts
        .iter()
        .filter(|&count| count.red <= max_red && count.blue <= max_blue && count.green <= max_green)
        .collect();

    valid_counts.len() == game.counts.len()
}

fn min_possible_counts(counts: Vec<Count>) -> Count {
    let mut min_red: u32 = 0;
    let mut min_blue: u32 = 0;
    let mut min_green: u32 = 0;

    for count in counts {
        min_red = min_red.max(count.red);
        min_blue = min_blue.max(count.blue);
        min_green = min_green.max(count.green);
    }

    Count {
        red: min_red,
        blue: min_blue,
        green: min_green,
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
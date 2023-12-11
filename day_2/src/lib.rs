use std::collections::HashMap;

fn number_in_string(s: &str) -> usize {
    let num = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    num.parse::<usize>().unwrap_or(0)
}

fn get_max_allowed(color: &str) -> usize {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0
    }
}

pub fn part_1(input: &str) -> usize {
    input.lines().map(|line| {
        let game = line.split(":").collect::<Vec<&str>>();
        let game_id = number_in_string(game[0]);

        for draw in game[1].split(";") {
            for color_count in draw.split(",") {
                let color = color_count.split(" ").collect::<Vec<&str>>().last().unwrap().to_owned();
                let count = number_in_string(color_count);
                if count > get_max_allowed(color) {
                    return 0;
                }
            }
        }
        game_id
    }).sum()
}

pub fn part_2(input: &str) -> usize {
    input.lines().map(|line| {
        println!("{}",line);
        let mut stores: HashMap<&str, usize> = HashMap::new();
        stores.insert("blue", 0);
        stores.insert("green", 0);
        stores.insert("red", 0);

        let game = line.split(":").collect::<Vec<&str>>();
        for draw in game[1].split(";") {
            for color_count in draw.split(",") {
                let color = color_count.split(" ").collect::<Vec<&str>>().last().unwrap().to_owned();
                let count = number_in_string(color_count);
                let max_count = stores.entry(color).or_insert(count).to_owned();
                if max_count < count {
                    stores.insert(color, count);
                }
            }
        }
        stores.values().product::<usize>()
    }).sum()
    }



#[cfg(test)]
mod day_2_tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(include_str!("../part1-example.txt")), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(include_str!("../part1-example.txt")), 2286);
    }
}


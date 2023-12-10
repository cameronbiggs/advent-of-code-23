use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    input.lines().map(|line| {
        let digits = line.chars().filter(|character| character.is_ascii_digit()).collect::<Vec<char>>();
        let first = digits.first().unwrap_or(&'0').to_owned();
        let last = digits.last().unwrap_or(&'0').to_owned();
        let value: usize = format!("{}{}", first, last).parse().expect("This should have been a number");
        value
    }).sum()
}

struct Node {
    value: String,
    children: HashMap<char, Node>
}

impl Node {
    fn new(value: String) -> Self {
        Node {
            value,
            children: HashMap::new()
        }
    }

    fn add_child(&mut self, key: char, value: String) -> &mut Node {
        self.children.entry(key.clone()).or_insert(Node::new(value))
    }
}

fn find_first_num_in_string(source_node: Node, sequence: &str, numbers: [&str; 9]) -> usize {
    let mut valid_nodes: Vec<&Node> = Vec::new();
    for c in sequence.chars() {
        if c.is_ascii_digit() {
            return usize::try_from(c.to_digit(10).expect("Not a number")).unwrap()
        }
        let mut next_loop_nodes: Vec<&Node> = Vec::new();
        valid_nodes.push(&source_node);
        for n in &valid_nodes {
            if n.children.contains_key(&c) {
                if let Some(new_node) = n.children.get(&c) {
                    if new_node.children.is_empty() {
                        return numbers.iter().position(|r| r == &new_node.value).unwrap() + 1
                    } else {
                        next_loop_nodes.push(&new_node);
                    }
                }
            }
        }
        valid_nodes = next_loop_nodes.clone()
    };
    return 0
}

pub fn part_2(input: &str) -> usize {
    input.lines().map(|line| {
        let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        let mut source = Node::new("Source".to_string());
        for num in numbers {
            let mut node = &mut source;
            for c in num.chars() {
                node = node.add_child(c, num.to_string());
            }
        }

        let mut back_source = Node::new("Source".to_string());
        let back_line = &line.chars().rev().collect::<String>();
        for num in numbers {
            let mut node = &mut back_source;
            for c in num.chars().rev() {
                node = node.add_child(c, num.to_string());
            }
        }

        let first =  find_first_num_in_string(source, line, numbers);
        let last = find_first_num_in_string(back_source, back_line, numbers);
        let value: usize = format!("{}{}", first, last).parse().expect("This should have been a number");
        value
    }).sum()
}

#[cfg(test)]
mod day_1_tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(include_str!("../part1-example.txt")), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(include_str!("../part2-example.txt")), 281);
    }
}

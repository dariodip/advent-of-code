use std::collections::{HashMap, HashSet};

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let graph = Graph::parse(input.text)?;

    Ok(graph.travel(Node::Start, Vec::new(), 0, input.part_values(true, false)))
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

#[derive(Default, Debug)]
struct Graph {
    nodes: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn parse(text: &str) -> Result<Self, String> {
        let mut graph: Graph = Default::default();

        for l in clean_lines(text).into_iter() {
            let (from, to) = l
                .split_once('-')
                .ok_or(format!("Cannot split line {}", l))?;
            let from_node = Node::parse(from.to_string())?;
            let to_node = Node::parse(to.to_string())?;
            graph
                .nodes
                .entry(from_node.clone())
                .or_default()
                .insert(to_node.clone());
            graph
                .nodes
                .entry(to_node.clone())
                .or_default()
                .insert(from_node.clone());
        }
        Ok(graph)
    }

    fn travel(&self, start: Node, visited: Vec<Node>, depth: u32, visited_twice: bool) -> u32 {
        if start == Node::End {
            return depth + 1;
        }

        let mut visited_twice = visited_twice;
        if !matches!(start, Node::Big(_)) {
            if start == Node::Start && !visited.is_empty()
                || visited_twice && visited.contains(&start)
            {
                return depth;
            } else if visited.contains(&start) {
                visited_twice = true;
            }
        }

        let mut visited = visited;
        visited.push(start.clone());
        depth
            + self
                .nodes
                .get(&start)
                .unwrap()
                .iter()
                .map(|n| self.travel(n.clone(), visited.clone(), depth, visited_twice))
                .sum::<u32>()
    }
}

impl Node {
    fn parse(node: String) -> Result<Self, String> {
        match node.as_str() {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            node if node.to_uppercase() == node => Ok(Self::Big(node.to_string())),
            node if node.to_lowercase() == node => Ok(Self::Small(node.to_string())),
            invalid_node => Err(format!("Cannot parse node {}", invalid_node)),
        }
    }
}

fn clean_lines(text: &str) -> Vec<&str> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect()
}

#[test]
pub fn test_example() -> Result<(), String> {
    use crate::input::{test_part_one, test_part_two};
    let example_input = r#"start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end"#;

    test_part_one!(example_input => 10);
    test_part_two!(example_input => 36);

    Ok(())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let file_input = include_str!("day12_input.txt");
    test_part_one!(file_input => 5920);
    test_part_two!(file_input => 155477);
}

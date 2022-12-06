#![allow(dead_code)]
use std::fs;
use petgraph::graph::UnGraph;
use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
struct Node {
    val: u128,
    index: usize,
}

impl Node {
    fn new(val: u128, index: usize) -> Node {
        Node {
            val,
            index
        }
    }
}

impl Into<NodeIndex> for Node {
    fn into(self) -> NodeIndex {
        NodeIndex::new(self.index)
    }
}

fn parse_input(test: bool) -> Vec<Vec<Node>> {
    let file_content = if test {
        fs::read_to_string("input/year2021/test15.txt").unwrap()
    } else {
        fs::read_to_string("input/year2021/day15.txt").unwrap()
    };

    let mut result = Vec::new();
    let mut counter = 0;
    for line in file_content.lines() {
        let mut v: Vec<Node> = Vec::new();
        for c in line.chars() {
            v.push(Node::new(c.to_string().parse().unwrap(), counter));
            counter += 1;
        }
        result.push(v);
    }

    return result;
}

fn create_graph(vec: Vec<Vec<Node>>) -> UnGraph<Node, u128> {
    let mut edges = Vec::new();

    // Create vec of pairs of Nodes
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            let hori_pair = match vec.get(i).unwrap().get(j+1) {
                Some(num) => Some((vec[i][j], *num, num.val as u128)),
                None => None,
            };
            let verti_pair = match vec.get(i+1) {
                Some(line_under) => Some((vec[i][j], line_under[j], line_under[j].val as u128)),
                None => None,
            };

            if let Some(pair) = hori_pair {
                edges.push(pair);
            }
            if let Some(pair) = verti_pair {
                edges.push(pair);
            }
        }
    }

    UnGraph::<Node, u128>::from_edges(edges)
}

pub fn part_one() {
    let input = parse_input(true);

    let (first, last) = (
        input.first().unwrap().first().unwrap().clone(), 
        input.last().unwrap().last().unwrap().clone()
    );

    let graph = create_graph(input);

    let node_map = dijkstra(
        &graph, 
        NodeIndex::new(first.index), 
        Some(NodeIndex::new(last.index)),
    |e| *e.weight(),
    );

    println!("The minimum way is {}", node_map.get(&NodeIndex::new(last.index)).unwrap());
}
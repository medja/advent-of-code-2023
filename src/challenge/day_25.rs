use rustc_hash::FxHashMap;
use std::{
    collections::{hash_map::Entry, VecDeque},
    hash::Hash,
    mem::MaybeUninit,
};

// The problem states that the minimum cut contains 3 edges
// It also implies that the minimum cut splits the graph into 2 similarly sized graphs
// So the graph is composed of 2 clusters of nodes, connected via 3 edges.
// This means that theres's a rougly 50% chance (turns out it's lower, but it still works out)
// that 2 randomly selected nodes will be part of different clusters
// Assuming there lots of paths between the nodes inside each cluster, that means that the
// most common edges (in paths between all node pairs) are going to be the edges we have to cut
// So this solution searches for the shortest path between 50 random nodes and determines the most
// frequent eges
// It then removes those edges and counts the number of reachable nodes (starting from one node)
// and if the number is smaller than the number of nodes, we've found the minimum cut
// Otherwise we haven't found the minimum cut and the solution searches for 10 more shortest paths
// It continues searching for 10 more paths until the solution is found
// This is technically a Monte Carlo algorith - it works well because it's very easy to determine if
// the solution is correct, so it can simply be repeated until the correct solution is found
pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut graph = build_graph(input);
    let mut frequencies = Frequencies::default();

    if let Some(result) = find_min_cut(50, &mut graph, &mut frequencies) {
        return Ok(result);
    }

    loop {
        if let Some(result) = find_min_cut(10, &mut graph, &mut frequencies) {
            return Ok(result);
        }
    }
}

fn build_graph(input: &[&str]) -> Vec<Node> {
    let mut ids = FxHashMap::default();
    let mut graph = Vec::new();

    for line in input {
        let line = line.as_bytes();
        let from = get_id(&line[..3], &mut ids, &mut graph);

        for to in line[5..].split(|char| *char == b' ') {
            let to = get_id(to, &mut ids, &mut graph);
            graph[from].edges.push(to);
            graph[to].edges.push(from);
        }
    }

    graph
}

fn get_id<'a>(
    value: &'a [u8],
    ids: &mut FxHashMap<&'a [u8], usize>,
    graph: &mut Vec<Node>,
) -> usize {
    let entry = match ids.entry(value) {
        Entry::Occupied(entry) => return *entry.get(),
        Entry::Vacant(entry) => entry,
    };

    let next_id = graph.len();
    entry.insert(next_id);
    graph.push(Node::default());
    next_id
}

fn find_min_cut(
    steps: usize,
    graph: &mut [Node],
    frequencies: &mut Frequencies<Edge>,
) -> Option<usize> {
    let mut rng = rand::thread_rng();

    for _ in 0..steps {
        let (from, to) = get_node_pair(&mut rng, graph.len());

        for edge in find_path(from, to, graph) {
            frequencies.add(edge);
        }
    }

    for edge in frequencies.most_frequent() {
        graph.get_mut(edge.from).unwrap().remove_edge(edge.to);
        graph.get_mut(edge.to).unwrap().remove_edge(edge.from);
    }

    let size = find_size(0, graph);

    if size != graph.len() {
        return Some(size * (graph.len() - size));
    }

    for edge in frequencies.most_frequent() {
        graph.get_mut(edge.from).unwrap().edges.push(edge.to);
        graph.get_mut(edge.to).unwrap().edges.push(edge.from);
    }

    None
}

// BFS search of shortest path between from and to
fn find_path(from: usize, to: usize, graph: &mut [Node]) -> impl Iterator<Item = Edge> + '_ {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_front((from, None));

    while let Some((node, parent)) = queue.pop_front() {
        visited[node] = true;

        if let Some(parent) = parent {
            graph[node].parent = Some(parent);
        }

        if node == to {
            break;
        }

        for &edge in &graph[node].edges {
            if !visited[edge] {
                queue.push_back((edge, Some(node)));
            }
        }
    }

    PathIterator {
        node: to,
        end: from,
        graph,
    }
}

// DFS count of accessible nodes
fn find_size(node: usize, graph: &[Node]) -> usize {
    let mut size = 0;
    let mut visited = vec![false; graph.len()];
    let mut queue = vec![node];

    while let Some(node) = queue.pop() {
        if visited[node] {
            continue;
        }

        size += 1;
        visited[node] = true;

        for &edge in &graph[node].edges {
            if !visited[edge] {
                queue.push(edge);
            }
        }
    }

    size
}

fn get_node_pair(rng: &mut impl rand::Rng, length: usize) -> (usize, usize) {
    let first = rng.gen_range(0..length);

    loop {
        let second = rng.gen_range(0..length);

        if first != second {
            break (first, second);
        }
    }
}

#[derive(Default)]
struct Node {
    edges: Vec<usize>,
    parent: Option<usize>,
}

impl Node {
    fn remove_edge(&mut self, edge: usize) {
        if let Some(index) = self.edges.iter().position(|value| *value == edge) {
            self.edges.swap_remove(index);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Default)]
struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    fn new(from: usize, to: usize) -> Self {
        if from <= to {
            Self { from, to }
        } else {
            Self::new(to, from)
        }
    }
}

struct PathIterator<'a> {
    node: usize,
    end: usize,
    graph: &'a [Node],
}

impl Iterator for PathIterator<'_> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node == self.end {
            return None;
        }

        let parent = self.graph[self.node].parent?;
        let edge = Edge::new(self.node, parent);
        self.node = parent;
        Some(edge)
    }
}

#[derive(Default)]
struct Frequencies<T>(FxHashMap<T, usize>);

impl<T: Clone + Eq + PartialEq + Hash> Frequencies<T> {
    fn most_frequent(&self) -> [&T; 3] {
        assert!(self.0.len() >= 3);

        let mut top_1_value = MaybeUninit::uninit();
        let mut top_1_count = 0;
        let mut top_2_value = MaybeUninit::uninit();
        let mut top_2_count = 0;
        let mut top_3_value = MaybeUninit::uninit();
        let mut top_3_count = 0;

        for (value, &count) in self.0.iter() {
            if count > top_1_count {
                (top_3_value, top_3_count) = (top_2_value, top_2_count);
                (top_2_value, top_2_count) = (top_1_value, top_1_count);
                (top_1_value, top_1_count) = (MaybeUninit::new(value), count);
            } else if count > top_2_count {
                (top_3_value, top_3_count) = (top_2_value, top_2_count);
                (top_2_value, top_2_count) = (MaybeUninit::new(value), count);
            } else if count > top_3_count {
                (top_3_value, top_3_count) = (MaybeUninit::new(value), count);
            }
        }

        unsafe {
            [
                top_1_value.assume_init(),
                top_2_value.assume_init(),
                top_3_value.assume_init(),
            ]
        }
    }

    fn add(&mut self, value: T) {
        match self.0.entry(value.clone()) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }
}

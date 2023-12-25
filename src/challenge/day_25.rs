use rand::Rng;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{hash_map::Entry, VecDeque};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut graph = FxHashMap::default();

    for line in input {
        let from = &line[..3];

        for to in line[5..].split_ascii_whitespace() {
            add_edge(from, to, &mut graph);
            add_edge(to, from, &mut graph);
        }
    }

    let keys = graph.keys().cloned().collect::<Vec<_>>();
    let mut frequencies = FxHashMap::default();

    for _ in 0..100 {
        let from = rand::thread_rng().gen_range(0..keys.len());

        let to = loop {
            let to = rand::thread_rng().gen_range(0..keys.len());

            if from != to {
                break to;
            }
        };

        for node in find_path(keys[from], keys[to], &mut graph) {
            match frequencies.entry(node) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }
    }

    let mut frequencies = frequencies.into_iter().collect::<Vec<_>>();
    frequencies.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (edge, _) in frequencies.into_iter().take(3) {
        graph.get_mut(edge.from).unwrap().edges.remove(edge.to);
        graph.get_mut(edge.to).unwrap().edges.remove(edge.from);
    }

    let size = find_size(keys[0], &graph);
    Ok(size * (graph.len() - size))
}

fn add_edge<'a>(from: &'a str, to: &'a str, graph: &mut FxHashMap<&'a str, Node<'a>>) {
    match graph.entry(from) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().edges.insert(to);
        }
        Entry::Vacant(entry) => {
            entry.insert(Node::new(to));
        }
    }
}

fn find_path<'a>(
    from: &'a str,
    to: &'a str,
    graph: &mut FxHashMap<&'a str, Node<'a>>,
) -> Vec<Edge<'a>> {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::<(&str, Option<&str>)>::new();
    queue.push_front((from, None));

    while let Some((node, parent)) = queue.pop_front() {
        visited.insert(node);

        if let Some(parent) = parent {
            graph.get_mut(node).unwrap().parent = Some(parent);
        }

        if node == to {
            break;
        }

        for edge in &graph.get(node).unwrap().edges {
            if !visited.contains(edge) {
                queue.push_back((edge, Some(node)));
            }
        }
    }

    let mut path = Vec::new();
    let mut node = to;

    while node != from {
        let parent = graph.get(node).unwrap().parent.unwrap();
        path.push(Edge::new(node, parent));
        node = parent;
    }

    path
}

fn find_size(node: &str, graph: &FxHashMap<&str, Node>) -> usize {
    let mut visited = FxHashSet::default();
    let mut queue = vec![node];

    while let Some(node) = queue.pop() {
        visited.insert(node);

        for edge in &graph.get(node).unwrap().edges {
            if !visited.contains(edge) {
                queue.push(edge);
            }
        }
    }

    visited.len()
}

#[derive(Debug)]
struct Node<'a> {
    edges: FxHashSet<&'a str>,
    parent: Option<&'a str>,
}

impl<'a> Node<'a> {
    fn new(edge: &'a str) -> Self {
        Self {
            edges: FxHashSet::from_iter([edge]),
            parent: None,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Edge<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> Edge<'a> {
    fn new(from: &'a str, to: &'a str) -> Self {
        if from <= to {
            Self { from, to }
        } else {
            Self::new(to, from)
        }
    }
}

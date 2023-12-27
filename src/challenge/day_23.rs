use crate::utils::IndexMapBuilder;
use rustc_hash::FxHashSet;
use std::ops::Index;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, true))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, false))
}

fn solve(input: &[&str], slopes: bool) -> usize {
    let maze = Maze::new(input, slopes);
    let graph = build_graph(maze);
    // Distance starts at 1 because the starting position is skipped
    find_longest_path(1, 1, &graph, &mut vec![false; graph.len()])
}

// DFS search of the optimized graph of the maze's junctions
fn find_longest_path(id: usize, distance: usize, graph: &[Node], visited: &mut [bool]) -> usize {
    if id == 0 {
        return distance;
    }

    visited[id] = true;
    let mut max = 0;

    for edge in graph[id].edges() {
        if visited[edge.id] {
            continue;
        }

        max = max.max(find_longest_path(
            edge.id,
            distance + edge.distance,
            graph,
            visited,
        ));
    }

    visited[id] = false;
    max
}

// Build a graph of the mazes junctions and the edges between them
fn build_graph(maze: Maze) -> Vec<Node> {
    let mut builder = IndexMapBuilder::<(usize, usize), Node>::default();
    builder.reserve(maze.end);

    // only keeps track of the points adjecent to junctions
    // the rest of the path doesn't need to be marked as visited
    let mut visited = FxHashSet::default();
    visited.insert((1, 0));

    // skips the starting position (x = 1, y = 0) to avoid requiring edge detection
    build_graph_node(1, 1, &mut builder, &mut visited, &maze);
    builder.build()
}

// DFS search of the entire maze
fn build_graph_node(
    x: usize,
    y: usize,
    builder: &mut IndexMapBuilder<(usize, usize), Node>,
    visited: &mut FxHashSet<(usize, usize)>,
    maze: &Maze,
) {
    let start_x = x;
    let start_y = y;
    let start_id = builder.find_index((x, y));

    for (direction, x, y) in find_neighbors(x, y) {
        // find unvisited paths
        if maze[(x, y)] == b'#' || visited.contains(&(x, y)) {
            continue;
        }

        // prevent re-entering the start of the path later
        visited.insert((x, y));

        // traverse the maze until the next junction
        let NextJunction {
            x,
            y,
            prev_x,
            prev_y,
            distance,
            traversable,
        } = match find_next_junction(direction, x, y, start_x, start_y, maze) {
            Some(junction) => junction,
            None => continue,
        };

        // prevent re-entering the end of the path later
        visited.insert((prev_x, prev_y));

        let end_id = builder.find_index((x, y));

        update_path(
            start_id,
            end_id,
            distance,
            traversable,
            builder.values_mut(),
        );

        // continue DFS from the newly discovered junction
        // the end of maze (id = 0) is not a junction
        if end_id != 0 {
            build_graph_node(x, y, builder, visited, maze);
        }
    }
}

fn find_neighbors(x: usize, y: usize) -> [(Direction, usize, usize); 4] {
    [
        (Direction::Right, x + 1, y),
        (Direction::Down, x, y + 1),
        (Direction::Left, x - 1, y),
        (Direction::Up, x, y - 1),
    ]
}

fn find_next_junction(
    mut direction: Direction,
    mut x: usize,
    mut y: usize,
    mut prev_x: usize,
    mut prev_y: usize,
    maze: &Maze,
) -> Option<NextJunction> {
    let mut distance = 1;
    let mut traversable = Traversable::Both;

    loop {
        if (x, y) == maze.end {
            break;
        }

        // keep track of which directions are traversable
        if maze.slopes {
            match maze[(x, y)] {
                b'>' if direction == Direction::Right => traversable = Traversable::Forward,
                b'>' => traversable = Traversable::Backward,
                b'v' if direction == Direction::Down => traversable = Traversable::Forward,
                b'v' => traversable = Traversable::Backward,
                _ => {}
            }
        }

        let mut steps = find_neighbors(x, y)
            .into_iter()
            .filter(|&(_, x, y)| maze[(x, y)] != b'#' && !(x == prev_x && y == prev_y));

        // determine if ...
        let next = match steps.next() {
            // there is only one path forward
            Some(next) if steps.next().is_none() => next,
            // there are multiple paths forward
            Some(_) => break,
            // this is a dead end
            None => return None,
        };

        (prev_x, prev_y) = (x, y);
        (direction, x, y) = next;
        distance += 1;
    }

    Some(NextJunction {
        x,
        y,
        prev_x,
        prev_y,
        distance,
        traversable,
    })
}

fn update_path(
    start_id: usize,
    end_id: usize,
    distance: usize,
    traversable: Traversable,
    graph: &mut [Node],
) {
    // if slopes are slippery the path might not be traversable in both directions
    match traversable {
        Traversable::Forward => {
            graph[start_id].insert_edge(Edge {
                id: end_id,
                distance,
            });
        }
        Traversable::Backward => {
            graph[end_id].insert_edge(Edge {
                id: start_id,
                distance,
            });
        }
        Traversable::Both => {
            graph[start_id].insert_edge(Edge {
                id: end_id,
                distance,
            });
            graph[end_id].insert_edge(Edge {
                id: start_id,
                distance,
            });
        }
    }
}

#[derive(Default)]
struct Node {
    edges: [Edge; 4],
    edge_count: usize,
}

impl Node {
    fn insert_edge(&mut self, edge: Edge) {
        let index = self.edge_count;
        self.edges[index] = edge;
        self.edge_count += 1;
    }

    fn edges(&self) -> &[Edge] {
        &self.edges[..self.edge_count]
    }
}

#[derive(Default)]
struct Edge {
    id: usize,
    distance: usize,
}

struct NextJunction {
    x: usize,
    y: usize,
    prev_x: usize,
    prev_y: usize,
    distance: usize,
    traversable: Traversable,
}

enum Traversable {
    Forward,
    Backward,
    Both,
}

#[derive(Eq, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Maze<'a> {
    grid: &'a [&'a str],
    slopes: bool,
    end: (usize, usize),
}

impl<'a> Maze<'a> {
    fn new(grid: &'a [&'a str], slopes: bool) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        let end = (width - 2, height - 1);

        Self { grid, slopes, end }
    }
}

impl Index<(usize, usize)> for Maze<'_> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1].as_bytes()[index.0]
    }
}

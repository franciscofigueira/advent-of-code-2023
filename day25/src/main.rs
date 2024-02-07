use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Read,
};

use rand::seq::SliceRandom;

#[derive(Debug)]
struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn add_edge(&mut self, component_a: &'a str, component_b: &'a str) {
        if let Some(edges) = self.edges.get_mut(component_a) {
            edges.push(component_b);
        } else {
            self.edges.insert(component_a, vec![component_b]);
        }
        if let Some(edges) = self.edges.get_mut(component_b) {
            edges.push(component_a);
        } else {
            self.edges.insert(component_b, vec![component_a]);
        }
    }

    fn remove_edge(&mut self, component_a: &str, component_b: &str) {
        let a = self.edges.get_mut(component_a).unwrap();
        let idx = a
            .iter()
            .enumerate()
            .find(|(_, &component)| component == component_b)
            .unwrap()
            .0;
        a.remove(idx);

        let b = self.edges.get_mut(component_b).unwrap();
        let idx = b
            .iter()
            .enumerate()
            .find(|(_, &component)| component == component_a)
            .unwrap()
            .0;
        b.remove(idx);
    }

    fn find_path(&self, origin: &'a str, dest: &'a str) -> Vec<&str> {
        let mut queue: VecDeque<(&str, Vec<&str>)> = VecDeque::new();

        queue.push_back((origin, vec![origin]));
        while let Some((current, seen)) = queue.pop_front() {
            if current == dest {
                return seen;
            }

            let next = self.edges.get(current).unwrap();
            next.iter()
                .filter(|&x| !seen.contains(x))
                .for_each(|&node| {
                    let mut nq = seen.clone();
                    nq.push(node);
                    queue.push_back((node, nq));
                })
        }
        return vec![];
    }

    fn num_reachable_nodes(&self, origin: &str) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(origin);

        while let Some(node) = queue.pop_front() {
            if visited.contains(node) {
                continue;
            }
            visited.insert(node);

            self.edges
                .get(node)
                .unwrap()
                .iter()
                .for_each(|&c| queue.push_back(c));
        }

        return visited.len();
    }
}

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut graph = Graph {
        edges: HashMap::new(),
    };

    let _ = contents
        .lines()
        .map(|line| {
            let (origin, connections) = line.split_once(":").expect("always have :");
            let connections = connections.split_whitespace();
            //println!("{origin}, {connections:?}");
            for connection in connections {
                // println!("{origin} -> {connection}");
                graph.add_edge(origin, connection);
            }
            return line;
        })
        .collect::<Vec<&str>>();

    let nodes: Vec<&str> = graph.edges.keys().map(|&x| x).collect();

    let mut edge_traversal_count: HashMap<String, usize> = HashMap::new();

    for _ in 0..500 {
        let mut chosen = nodes.choose_multiple(&mut rand::thread_rng(), 2);
        let origin = *chosen.next().unwrap();
        let dest = *chosen.next().unwrap();

        graph.find_path(origin, dest).iter().for_each(|&n| {
            let count = edge_traversal_count.get(n).unwrap_or(&0);
            edge_traversal_count.insert(n.to_string(), count + 1);
        })
    }

    let mut count_vecs: Vec<_> = edge_traversal_count.iter().collect();
    count_vecs.sort_by(|a, b| b.1.cmp(a.1));
    //println!("{count_vecs:?}");
    let mut suspicious: Vec<_> = count_vecs.iter().take(6).map(|x| x.0.clone()).collect();

    let mut a: String = String::new();
    let mut b: String = String::new();
    // let b: &str;
    // let mut first = true;

    while let Some(node) = suspicious.pop() {
        let connections = graph.edges.get(node.as_str()).unwrap().clone();
        for n in suspicious.clone() {
            if connections.contains(&n.as_str()) {
                a = node.clone();
                b = n.clone();
                graph.remove_edge(node.as_str(), n.as_str());
                let idx = suspicious
                    .iter()
                    .enumerate()
                    .find(|(_, x)| x.clone() == &n)
                    .unwrap()
                    .0;
                suspicious.remove(idx);
                println!("connection,{node} , {n} removed");
                break;
            }
        }
    }

    let group_1 = graph.num_reachable_nodes(&a);
    let group_2 = graph.num_reachable_nodes(&b);
    let result = group_1 * group_2;

    println!("group 1 size {group_1}, group 2 size {group_2}, reuslt {result}");
}

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    rc::{Rc, Weak},
};

use crate::read::Solution;

const END_LABEL: &str = "out";

type NodePtr = *const RefCell<Node>;
type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;
// type Path = Vec<NodeRef>;
#[derive(Clone)]
struct Path(Vec<NodeRef>);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|n| n.borrow().label.clone())
                .collect::<Vec<_>>()
                .join(" > ")
        )
    }
}

#[derive(Debug)]
pub struct Graph {
    pub start_label: &'static str,
    pub nodes: HashMap<String, NodeRef>,
    pub start_nodes: Vec<NodeRef>,
}

impl Graph {
    fn new(start_label: &'static str) -> Self {
        return Graph {
            start_label,
            nodes: HashMap::new(),
            start_nodes: vec![],
        };
    }

    fn get_node(&self, label: &str) -> Option<&NodeRef> {
        self.nodes.get(label)
    }

    fn add_node(&mut self, node: Node) -> NodeRef {
        let entry = self
            .nodes
            .entry(node.label.clone())
            .or_insert(node.to_ref());

        return Rc::clone(entry);
    }

    fn get_or_create_node<F>(&mut self, label: &str, create: F) -> NodeRef
    where
        F: FnOnce() -> Node,
    {
        if let Some(existing) = self.get_node(label) {
            return Rc::clone(existing);
        }

        let node_ref = create().to_ref();
        self.nodes.insert(label.to_string(), Rc::clone(&node_ref));

        if node_ref.borrow().is_start(self.start_label) {
            self.start_nodes.push(Rc::clone(&node_ref));
        }

        return node_ref;
    }

    fn find_paths_back(&self, pass_through_req: Option<&HashSet<NodePtr>>) -> u128 {
        let order = self.kahn();

        // println!(
        //     "{}",
        //     order
        //         .iter()
        //         .map(|n| n.borrow().label.clone())
        //         .fold(String::new(), |acc, l| acc + " -> " + &l)
        // );
        // let pass_through_ix: HashMap<NodePtr, usize> = pass_through_req
        //     .unwrap_or(&HashSet::new())
        //     .iter()
        //     .enumerate()
        //     .map(|(i, node)| (*node, i))
        //     .collect();
        let mut memory: HashMap<NodePtr, (HashSet<NodePtr>, u128)> = HashMap::new();

        for node in order.iter().rev() {
            let ptr = Rc::as_ptr(node);

            if node.borrow().is_end() {
                memory.insert(ptr, (HashSet::new(), 1u128));
            }

            if node.borrow().label == "nuk" {
                println!();
            }

            let default_mem = (HashSet::new(), 0u128);

            let mut node_mem = memory.get(&ptr).unwrap_or(&default_mem).to_owned();

            if let Some(pass_through_req) = pass_through_req
                && pass_through_req.contains(&ptr)
            {
                node_mem.0.insert(ptr);
            }

            if node.borrow().is_start(self.start_label) {
                if let Some(pass_through_req) = pass_through_req
                    && pass_through_req.len() != node_mem.0.len()
                {
                    return 0;
                }

                return node_mem.1;
            }

            for prev_node in &node.borrow().prev {
                if let Some(prev_node) = prev_node.upgrade() {
                    let ptr = Rc::as_ptr(&prev_node);

                    if let Some(v) = memory.get_mut(&ptr) {
                        if node_mem.0.len() == v.0.len() {
                            v.1 += node_mem.1;
                        } else if node_mem.0.len() > v.0.len() {
                            memory.insert(ptr, node_mem.clone());
                        }
                    } else {
                        memory.insert(ptr, node_mem.clone());
                    }
                }
            }
        }

        return 0u128;
    }

    fn find_paths(&self, pass_through_req: Option<&HashSet<NodePtr>>) -> Vec<Path> {
        let mut result: Vec<Path> = vec![];

        for start_node in &self.start_nodes {
            self.dfs(
                start_node,
                &mut Path(vec![]),
                &mut HashSet::new(),
                &mut result,
                &mut HashSet::new(),
                pass_through_req,
            );
        }

        return result;
    }

    fn dfs(
        &self,
        node_ref: &NodeRef,
        path: &mut Path,
        visited: &mut HashSet<NodePtr>,
        result: &mut Vec<Path>,
        pass_through: &mut HashSet<NodePtr>,
        pass_through_req: Option<&HashSet<NodePtr>>,
    ) {
        let ptr = Rc::as_ptr(&node_ref);

        if visited.contains(&ptr) {
            return;
        }

        if let Some(pass_through_req) = pass_through_req
            && pass_through_req.contains(&ptr)
        {
            pass_through.insert(ptr);
        }

        visited.insert(ptr);
        path.0.push(Rc::clone(&node_ref));

        if node_ref.borrow().is_end() {
            let mut push_path = true;
            if let Some(pass_through_req) = pass_through_req {
                push_path = pass_through.len() == pass_through_req.len()
            }

            if push_path {
                result.push(path.clone());
                println!("{}", path);
            }
        } else {
            for next_ref in &node_ref.borrow().next {
                self.dfs(
                    next_ref,
                    path,
                    visited,
                    result,
                    pass_through,
                    pass_through_req,
                );
            }
        }

        path.0.pop();
        visited.remove(&ptr);
        if pass_through_req.is_some() {
            pass_through.remove(&ptr);
        }
    }

    fn kahn(&self) -> Vec<NodeRef> {
        let mut n_inputs: HashMap<NodePtr, usize> = HashMap::new();
        let mut queue: VecDeque<NodeRef> = VecDeque::new();
        let mut visited: HashSet<NodePtr> = HashSet::new();
        let mut order: Vec<NodeRef> = vec![];

        for (_, node) in &self.nodes {
            let ptr = Rc::as_ptr(node);
            let prev_size = node.borrow().prev.len();
            n_inputs.insert(ptr, prev_size);
            if prev_size == 0 {
                queue.push_back(Rc::clone(node));
            }
        }

        while let Some(next) = queue.pop_front() {
            for adj in &next.borrow().next {
                let adj_ptr = Rc::as_ptr(&adj);
                n_inputs.entry(adj_ptr).and_modify(|v| *v -= 1);

                if let Some(n) = n_inputs.get(&adj_ptr)
                    && *n == 0
                {
                    queue.push_back(Rc::clone(adj));
                }
            }

            let next_ptr = Rc::as_ptr(&next);
            order.push(next);
            visited.insert(next_ptr);
        }

        println!(
            "visited {}, queue {}, nodes {} ",
            visited.len(),
            queue.len(),
            self.nodes.len()
        );

        return order;
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}\n=============", node.1.borrow())?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub next: Vec<NodeRef>,
    pub prev: Vec<WeakNodeRef>,
}

impl Node {
    fn new(label: String) -> Self {
        return Node {
            label,
            next: vec![],
            prev: vec![],
        };
    }

    fn to_ref(self) -> NodeRef {
        return Rc::new(RefCell::new(self));
    }

    fn is_start(&self, start_label: &'static str) -> bool {
        return self.label == start_label;
    }

    fn is_end(&self) -> bool {
        return self.label == END_LABEL;
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // println!("dropping {}", self.label);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " \\/- {}\n {}\n /\\- {}",
            self.prev
                .iter()
                .map(|n| n.upgrade())
                .filter_map(|x| if let Some(v) = x {
                    Some(v.borrow().label.clone())
                } else {
                    None
                })
                .collect::<Vec<_>>()
                .join(", "),
            self.label,
            self.next
                .iter()
                .map(|n| n.borrow().label.clone())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn parse(input: &str, start_label: &'static str) -> Graph {
    let mut graph = Graph::new(start_label);
    for line in input.lines().into_iter() {
        let (label, adjacent) = line.split_once(':').unwrap();
        let node_ref = graph.get_or_create_node(label, || Node::new(label.to_string()));

        for next in adjacent.trim().split_whitespace() {
            let next_noderef = graph.get_or_create_node(next, || Node::new(next.to_string()));

            next_noderef
                .borrow_mut()
                .prev
                .push(Rc::downgrade(&node_ref));
            node_ref.borrow_mut().next.push(next_noderef);
        }
    }

    return graph;
}

pub struct PartA {
    graph: Graph,
}

impl Solution for PartA {
    fn new(input: &str) -> Self {
        let graph = parse(input, "you");
        PartA { graph }
    }

    fn execute(&self) {
        let graph = &self.graph;
        let paths: Vec<Path> = graph.find_paths(None);

        println!("paths length {:}", paths.len());
    }
}

pub struct PartB {
    graph: Graph,
}

impl Solution for PartB {
    fn new(input: &str) -> Self {
        let graph = parse(input, "svr");

        PartB { graph }
    }

    fn execute(&self) {
        let graph = &self.graph;
        let pass_through_labels = ["dac", "fft"];

        let pass_through_nodes = pass_through_labels
            .iter()
            .map(|l| {
                graph
                    .get_node(l)
                    .expect(&format!("pass through node \"{}\" not found", l))
            })
            .fold(
                HashSet::<NodePtr>::with_capacity(pass_through_labels.len()),
                |mut acc, x| {
                    acc.insert(Rc::as_ptr(x));
                    acc
                },
            );

        // let paths = graph.find_paths(Some(&pass_through_nodes));
        // let paths = graph.find_paths(None);

        // println!("paths length {}", paths.len());

        let r = graph.find_paths_back(Some(&pass_through_nodes));

        println!("{}", r);
    }
}

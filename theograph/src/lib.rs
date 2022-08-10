#![allow(dead_code)]
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Node<T: PartialEq + Eq + Hash> {
    content: T,
}

impl<T: PartialEq + Eq + Hash> Node<T> {
    pub fn new(content: T) -> Node<T> {
        Node { content }
    }

    pub fn set_content(&mut self, new_content: T) {
        self.content = new_content;
    }

    pub fn get_content(&self) -> &T {
        &self.content
    }

    pub fn wrap(self) -> NodeWrapper<T> {
        NodeWrapper { node: Rc::new(RefCell::new(self)) }
    }
}


#[derive(PartialEq, Eq)]
struct NodeWrapper<T: PartialEq + Eq + Hash> {
    node: Rc<RefCell<Node<T>>>
}

impl<T: PartialEq + Eq + Hash> NodeWrapper<T> {
    fn new(n: Rc<RefCell<Node<T>>>) -> NodeWrapper<T> {
        NodeWrapper { node: n }
    }
}

impl<T: PartialEq + Eq + Hash> Hash for NodeWrapper<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.borrow().content.hash(state);
    }
}


#[derive(Eq)]
struct Edge<T: PartialEq + Eq + Hash> {
    left: Rc<RefCell<Node<T>>>,
    right: Rc<RefCell<Node<T>>>,
}

impl<T: PartialEq + Eq + Hash> Edge<T> {
    pub fn new(left: Rc<RefCell<Node<T>>>, right: Rc<RefCell<Node<T>>>) -> Option<Edge<T>> {
        if left.borrow().get_content() == right.borrow().get_content() {
            return None;
        }

        Some (
            Edge { left, right }
        )
    }

    pub fn get_nodes(&self) -> (Weak<RefCell<Node<T>>>, Weak<RefCell<Node<T>>>) {
        (
            Rc::downgrade(&self.left),
            Rc::downgrade(&self.right),
        )
    }
}

impl<T: PartialEq + Eq + Hash> Hash for Edge<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.left.borrow().hash(state);
        self.right.borrow().hash(state);
    }
}

impl<T: PartialEq + Eq + Hash> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.left.borrow().get_content() == other.left.borrow().get_content() 
              && self.right.borrow().get_content() == other.right.borrow().get_content() {
            return true;
        }

        if self.left.borrow().get_content() == other.right.borrow().get_content() 
              && self.right.borrow().get_content() == other.left.borrow().get_content() {
            return true;
        }

        false
    }
}



struct Graph<N: PartialEq + Eq + Hash> {
    nodes: HashSet<NodeWrapper<N>>,
    edges: HashSet<Edge<N>>
}

impl<N: PartialEq + Eq + Hash> Graph<N> {
    pub fn new() -> Graph<N> {
        Graph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, n: Node<N>) {
        self.add_node_wrapper(n.wrap());
    }

    pub fn add_node_wrapper(&mut self, n: NodeWrapper<N>) {
        if self.nodes.contains(&n) {
            return;
        }

        self.nodes.insert(n);
    }

    pub fn add_edge(&mut self, e: Edge<N>) {
        let left_wrapper = NodeWrapper::new(Rc::clone(&e.left));
        let right_wrapper = NodeWrapper::new(Rc::clone(&e.right));

        self.add_node_wrapper(left_wrapper);
        self.add_node_wrapper(right_wrapper);

        if self.edges.contains(&e) {
            return;
        }

        self.edges.insert(e);
    }

    pub fn node_amount(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_amoun(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Graph<i32> {
        let mut graph = Graph::new();

        graph.add_node(Node::new(1));
        graph.add_node(Node::new(2));
        graph.add_node(Node::new(3));
        graph.add_node(Node::new(2));

        graph
    }

    #[test]
    fn add_node() {
        let graph = setup();

        assert_eq!(graph.node_amount(), 3);
    }

    #[test]
    fn add_edges() {

    }
}

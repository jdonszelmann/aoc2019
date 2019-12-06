use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct OrbitTree<'o> {
    tree: Rc<RefCell<Tree<'o>>>,
    nodes: HashMap<&'o str, Rc<RefCell<Tree<'o>>>>,
}

impl<'o> OrbitTree<'o> {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Tree::with_value("COM")));

        let mut map = HashMap::new();
        map.insert("COM", root.clone());

        Self {
            tree: root,
            nodes: map,
        }
    }

    /// Returns if could be inserted
    pub fn add_orbit(&mut self, id: &'o str, parent: &str) -> bool {
        let parent_node = match self.nodes.get(parent) {
            Some(i) => i,
            None => return false,
        }
        .clone();

        let parent_count = parent_node.borrow().count;
        let child = Rc::new(RefCell::new(Tree::new(parent_count, id)));

        self.nodes.insert(id, child.clone());

        parent_node.borrow_mut().add_child(child);

        true
    }

    pub fn leaf_path(&self, src: &str, dst: &str) -> Option<u64> {
        let mut curr = self.nodes.get(src)?;
        let mut iterations = 0;

        loop {
            iterations += 1;
            curr = self.nodes.get(curr.borrow().parent)?;
            if let Some(i) = Self::search_leaf(curr.borrow(), dst, 0) {
                return Some(i + iterations - 2);
            }
        }
    }

    pub fn search_leaf(start: Ref<Tree>, value: &str, distance: u64) -> Option<u64> {
        if start.value == value {
            return Some(distance);
        }

        for i in &start.children {
            if let Some(i) = Self::search_leaf(i.borrow(), value, distance + 1) {
                return Some(i);
            }
        }

        None
    }

    pub fn dfs(&self) -> u64 {
        self.tree.borrow().dfs()
    }
}

pub struct Tree<'t> {
    pub children: Vec<Rc<RefCell<Tree<'t>>>>,
    pub count: u64,
    pub parent: &'t str,
    pub value: &'t str,
}

impl<'t> Tree<'t> {
    pub fn new(parent_count: u64, value: &'t str) -> Self {
        Self {
            children: vec![],
            count: parent_count + 1,
            parent: "",
            value,
        }
    }

    pub fn with_value(value: &'t str) -> Self {
        Self {
            children: vec![],
            count: 0,
            parent: "",
            value,
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Tree<'t>>>) {
        child.borrow_mut().parent = self.value;
        self.children.push(child);
    }

    pub fn dfs(&self) -> u64 {
        let mut total = self.count;
        for i in &self.children {
            total += i.borrow().dfs();
        }
        total
    }
}

pub fn build_orbittree(input: &str) -> OrbitTree {
    let mut orbittree = OrbitTree::new();
    let mut uninserted_parts = HashMap::new();

    for i in input.lines() {
        let parts: Vec<&str> = i.split_terminator(')').collect();
        let parent = parts[0];
        let child = parts[1];

        if !orbittree.add_orbit(child, parent) {
            uninserted_parts.insert(child, parent);
        }
    }

    loop {
        let mut inserted = vec![];

        for (child, parent) in &uninserted_parts {
            if orbittree.add_orbit(child, parent) {
                //Cloning a reference is *actually* what I want here.
                //The reference is not owned by me but by cloning it it is.
                #[allow(clippy::clone_double_ref)]
                inserted.push(child.clone());
            }
        }

        for i in inserted {
            uninserted_parts.remove(i);
        }

        if uninserted_parts.is_empty() {
            break;
        }
    }

    orbittree
}

fn main_func(input: &str) -> u64 {
    let tree = build_orbittree(input);
    tree.dfs()
}

#[cfg(test)]
mod test {
    use crate::day06::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input);
        assert_eq!(result, 402879);
        println!("challenge 6.1: {}", result);
    }

    #[test]
    fn test_main_1() {
        assert_eq!(
            main_func("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
            42
        );
    }

    #[test]
    fn test_main_2() {
        assert_eq!(
            main_func("B)C\nB)G\nCOM)B\nC)D\nD)E\nE)F\nG)H\nE)J\nD)I\nJ)K\nK)L"),
            42
        );
    }
}

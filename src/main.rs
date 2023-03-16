use std::collections::HashMap;

#[derive(Debug)]
pub struct RadixTree<T> {
    children: HashMap<char, RadixTree<T>>,
    value: Option<T>,
}

impl<T> RadixTree<T> {
    pub fn new() -> Self {
        RadixTree {
            children: HashMap::new(),
            value: None,
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        if key.is_empty() {
            self.value = Some(value);
            return;
        }

        let first_char = key.chars().next().unwrap();
        if !self.children.contains_key(&first_char) {
            self.children.insert(first_char, RadixTree::new());
        }

        self.children
            .get_mut(&first_char)
            .unwrap()
            .insert(&key[1..], value);
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        if key.is_empty() {
            let value = self.value.take();
            if self.children.is_empty() && self.value.is_none() {
                None
            } else {
                value
            }
        } else {
            let first_char = key.chars().next().unwrap();
            if let Some(child) = self.children.get_mut(&first_char) {
                let result = child.remove(&key[1..]);
                if child.children.is_empty() && child.value.is_none() {
                    self.children.remove(&first_char);
                }
                result
            } else {
                None
            }
        }
    }

    pub fn find(&self, key: &str) -> Option<&T> {
        if key.is_empty() {
            self.value.as_ref()
        } else {
            let first_char = key.chars().next().unwrap();
            if let Some(child) = self.children.get(&first_char) {
                child.find(&key[1..])
            } else {
                None
            }
        }
    }

    pub fn search(&self, key: &str) -> Vec<&str> {
        let mut node = self;
        for c in key.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return vec![];
            }
        }

        let mut results = vec![];
        node.collect_descendants(key, &mut results);
        results
    }

    fn collect_descendants<'a>(&self, prefix: &'a str, results: &mut Vec<&'a str>) {
        if let Some(value) = &self.value {
            results.push(prefix);
        }

        for (c, child) in &self.children {
            let mut next_prefix = prefix.to_owned();
            next_prefix.push(*c);
            child.collect_descendants(&(next_prefix.clone()), results);
        }
    }
}

fn main() {
    let mut tree = RadixTree::new();
    tree.insert("bat", ());
    tree.insert("batter", ());
    tree.insert("battery", ());
    tree.insert("bathroom", ());
    tree.insert("bathrooms", ());
    tree.insert("hut", ());

    let input = "ba";
    let results = tree.search(input);
    println!("Search results for '{}': {:?}", input, results);

    let input = "bath";
    let results = tree.search(input);
    println!("Search results for '{}': {:?}", input, results);
}

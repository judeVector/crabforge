use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
struct HashNode {
    key: String,
    value: String,
    next: Option<Box<HashNode>>,
}

#[derive(Debug)]
struct HashTable {
    size: usize,
    array: Vec<Option<Box<HashNode>>>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        HashTable {
            size,
            array: vec![None; size],
        }
    }

    fn hash(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();

        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.size
    }

    fn insert(&mut self, key: String, value: String) {
        let index = self.hash(&key);

        let new_node = Box::new(HashNode {
            key,
            value,
            next: self.array[index].take(),
        });

        self.array[index] = Some(new_node)
    }

    fn get(&self, key: &str) -> Option<&str> {
        let index = self.hash(key);

        let mut current = self.array[index].as_ref();

        while let Some(node) = current {
            if node.key == key {
                return Some(&node.value);
            }
            current = node.next.as_ref().map(|n| n)
        }
        None
    }
}

fn main() {
    let mut data = HashTable::new(10);
    data.insert("Jude".into(), "Vector".into());
    let answer = data.get("Jude");

    println!("{:?}", data);
    println!("{}", answer.unwrap())
}

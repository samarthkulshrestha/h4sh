use std::time::Instant;


mod h4sh {
    use std::fmt::Debug;
    use std::cmp::PartialEq;


    trait Hashable {
        fn hash(&self) -> usize;
    }

    impl Hashable for String {
        // http://www.cse.yorku.ca/~oz/hash.html
        fn hash(&self) -> usize {
            let mut res: usize = 5381;
            for c in self.bytes() {
                res = ((res << 5).wrapping_add(res)).wrapping_add(c.into());
            }
            res
        }
    }

    impl Hashable for usize {
        fn hash(&self) -> usize {
            *self
        }
    }

#[derive(Default, Clone)]
    struct HashCell<Key, Value> {
        key: Key,
        value: Value,
        taken: bool,
    }

    pub struct HashTable<Key, Value> {
        cells: Vec<HashCell<Key, Value>>,
        taken_count: usize,
    }

    impl<Key, Value> HashTable<Key, Value>
    where
        Key: Default + Clone + Debug + PartialEq + Hashable,
        Value: Default + Clone + Debug
    {

        pub fn new() -> Self {
            const INITIAL_CAP: usize = 11;
            Self {
                cells: vec![HashCell::<_, _>::default(); INITIAL_CAP],
                taken_count: 0,
            }
        }

        fn debug_dump(&self) {
            for cell in self.cells.iter() {
                if cell.taken {
                    println!("{:?} -> {:?}", cell.key, cell.value);
                } else {
                    println!("x");
                }
            }
        }

        pub fn extend(&mut self) {
            assert!(self.cells.len() > 0);
            let mut new_self = Self {
                cells: vec![HashCell::<_, _>::default(); self.cells.len() * 2 + 1],
                taken_count: 0,
            };

            for cell in self.cells.iter() {
                if cell.taken {
                    new_self.insert(cell.key.clone(), cell.value.clone());
                }
            }

            *self = new_self;
        }

        pub fn insert(&mut self, key: Key, new_value: Value) {
            if let Some(old_value) = self.get_mut(&key) {
                *old_value = new_value;
            } else {
                if self.taken_count >= self.cells.len() {
                    self.extend();
                }
                assert!(self.taken_count < self.cells.len());

                let mut index = key.hash() % self.cells.len();

                while self.cells[index].taken {
                    index = (index + 1) % self.cells.len();
                }

                self.cells[index].taken = true;
                self.cells[index].key = key;
                self.cells[index].value = new_value;
                self.taken_count += 1;
            }
        }

        fn get_index(&self, key: &Key) -> Option<usize> {
            let mut index = key.hash() % self.cells.len();
            for _ in 0..self.cells.len() {
                if !self.cells[index].taken {
                    break;
                }

                if self.cells[index].key == *key {
                    break;
                }

                index = (index + 1) % self.cells.len();
            }

            if self.cells[index].taken && self.cells[index].key == *key {
                return Some(index);
            } else {
                None
            }
        }

        pub fn get(&self, key: &Key) -> Option<&Value> {
            self.get_index(key).map(|index| &self.cells[index].value)
        }

        pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
            self.get_index(key).map(|index| &mut self.cells[index].value)
        }
    }
}

fn benchmark_our_virgin_table(n: usize) {
    use h4sh::*;
    let mut hash = HashTable::<usize, usize>::new();
    for _ in 0..n {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
}

fn benchmark_std_chad_table(n: usize) {
    use std::collections::HashMap;

    let mut hash = HashMap::<usize, usize>::new();
    for _ in 0..n {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
}

fn main() {
    const N: usize = 100_000;

    let begin = Instant::now();
    benchmark_our_virgin_table(N);
    println!("virgin table: {}", begin.elapsed().as_secs_f32());

    let begin = Instant::now();
    benchmark_std_chad_table(N);
    println!("chad table: {}", begin.elapsed().as_secs_f32());
}

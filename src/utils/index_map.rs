use rustc_hash::FxHashMap;
use std::{
    collections::hash_map::Entry,
    hash::Hash,
    ops::{Index, IndexMut},
};

pub struct IndexMapBuilder<K, V> {
    index_lookup: FxHashMap<K, usize>,
    values: Vec<V>,
}

#[allow(dead_code)]
impl<K, V> IndexMapBuilder<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.index_lookup.len()
    }

    pub fn values(&self) -> &[V] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [V] {
        &mut self.values
    }

    pub fn build(mut self) -> Vec<V> {
        if self.index_lookup.len() < self.values.len() {
            self.values.truncate(self.index_lookup.len())
        }

        self.values
    }
}

#[allow(dead_code)]
impl<K, V: Clone + Default> IndexMapBuilder<K, V> {
    pub fn with_capacity(capacity: usize) -> Self {
        IndexMapBuilder {
            index_lookup: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            values: vec![V::default(); capacity],
        }
    }
}

#[allow(dead_code)]
impl<K: Eq + PartialEq + Hash, V: Default> IndexMapBuilder<K, V> {
    pub fn find_index(&mut self, key: K) -> usize {
        let next_index = self.index_lookup.len();

        let entry = match self.index_lookup.entry(key) {
            Entry::Occupied(entry) => return *entry.get(),
            Entry::Vacant(entry) => entry,
        };

        entry.insert(next_index);

        if self.values.len() <= next_index {
            self.values.push(V::default());
        }

        next_index
    }

    #[inline(always)]
    pub fn reserve(&mut self, key: K) {
        self.find_index(key);
    }
}

impl<K, V> Default for IndexMapBuilder<K, V> {
    fn default() -> Self {
        Self {
            index_lookup: Default::default(),
            values: Default::default(),
        }
    }
}

impl<K, V> Index<usize> for IndexMapBuilder<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<K, V> IndexMut<usize> for IndexMapBuilder<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

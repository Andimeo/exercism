use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Clone + Eq + Hash> {
    data: HashSet<T>,
}

impl<T: Clone + Eq + Hash> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            data: input.iter().cloned().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.data.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.is_subset(&other.data)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet {
            data: self.data.intersection(&other.data).cloned().collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet {
            data: self.data.difference(&other.data).cloned().collect(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet {
            data: self.data.union(&other.data).cloned().collect(),
        }
    }
}

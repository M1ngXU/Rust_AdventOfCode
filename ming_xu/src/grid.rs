use std::collections::HashMap;
pub struct GridElement<T> {
    pub coord: Vec<i64>,
    pub v: T
}

#[derive(Debug)]
pub struct Sparse<T: Copy> {
    data: HashMap<Vec<i64>, T>,
    dimensions: usize,
    default: T
}

impl<T: Copy> IntoIterator for Sparse<T> {
    type Item = GridElement<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values().into_iter()
    }
}

impl<T: Copy> Sparse<T> {
    /// Do these coordinates exist?
    pub fn contains(&self, coord: &Vec<i64>) -> bool {
        coord.len() == self.dimensions() && self.data.contains_key(coord)
    }

    /// counts the amount of elements that have 'true' returned in the closure
    pub fn count_if(&self, f: impl Fn(T) -> bool) -> usize {
        self.values().into_iter().filter(| g | f(g.v)).count()
    }

    /// returns the default value of the grid
    pub fn default(&self) -> T {
        self.default
    }

    /// returns the amount of dimensions of the grid
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    /// Get a immutable cell
    pub fn get(&self, coord: &Vec<i64>) -> Option<&T> {
        self.data.get(coord)
    }

    /// Get a mutable version for cell
    pub fn get_mut(&mut self, coord: &Vec<i64>) -> Option<&mut T> {
        self.data.get_mut(coord)
    }

    /// returns all elements (or default values) in a line, must be 0 or 45 degree
    pub fn edit_line(&mut self, start: &Vec<i64>, end: &Vec<i64>, edit_v: impl Fn(&mut T)) {
        if !self.in_dimensions(start) || !self.in_dimensions(end) {
            return;
        }
        let step = end.iter().enumerate().map(| (i, c) | (c - start[i]).signum()).collect::<Vec<i64>>();
        let mut current = start.clone();
        let mut last = false;
        loop {
            edit_v(self.get_mut_or_insert_default(&current));
            if last {
                break;
            }
            current.iter_mut().enumerate().filter(| (i, _) | step[*i] != 0).for_each(| (i, c) | {
                *c += step[i];
                if *c == end[i] {
                    last = true;
                }
            });
        }
    }

    /// Returns the mutable value of the cell after inserting the default value if non-existing cell
    pub fn get_mut_or_insert_default(&mut self, coord: &Vec<i64>) -> &mut T {
        self.get_mut_or_insert_value(coord, self.default())
    }

    /// Returns the mutable value of the cell after inserting a value if non-existing cell
    pub fn get_mut_or_insert_value(&mut self, coord: &Vec<i64>, v: T) -> &mut T {
        self.insert(coord, v);
        self.get_unwrapped_mut(coord)
    }

    /// Returns a unwrapped immutable cell, make sure that the cell exists!
    pub fn get_unwrapped(&self, coord: &Vec<i64>) -> T {
        *self.get(coord).unwrap()
    }

    /// Returns a unwrapped mutable cell, make sure that the cell exists!
    pub fn get_unwrapped_mut(&mut self, coord: &Vec<i64>) -> &mut T {
        self.get_mut(coord).unwrap()
    }

    pub fn in_dimensions(&mut self, coord: &Vec<i64>) -> bool {
        coord.len() == self.dimensions()
    }

    /// Inserts value into the cell x, y if there isn't any
    pub fn insert(&mut self, coord: &Vec<i64>, v: T) {
        if !self.contains(coord) {
            self.data.insert(coord.clone(), v);
        }
    }

    /// Inserts the default value into the cell if there isn't any
    pub fn insert_default(&mut self, coord: &Vec<i64>) {
        self.insert(coord, self.default());
    }

    /// creates a new 2d Grid, sets default value
    pub fn new(dimensions: usize, default: T) -> Self {
        Self {
            data: HashMap::new(),
            default,
            dimensions
        }
    }

    /// Replaces/Inserts value into the cell x, y
    pub fn replace(&mut self, coord: Vec<i64>, v: T) {
        self.data.insert(coord, v);
    }

    pub fn values(&self) -> Vec<GridElement<T>> {
        self.data.keys().map(| coord | GridElement {
            coord: coord.clone(),
            v: self.get_unwrapped(coord)
        }).collect()
    }
}
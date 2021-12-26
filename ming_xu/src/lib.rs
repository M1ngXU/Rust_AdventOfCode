pub mod grid {
    use std::collections::HashMap;

    pub struct GridElement<T> {
        pub x: i64,
        pub y: i64,
        pub v: T
    }

    #[derive(Debug)]
    pub struct Sparse2d<T: Copy> {
        data: HashMap<i64, HashMap<i64, T>>,
        default: T
    }

    impl<T: Copy> IntoIterator for Sparse2d<T> {
        type Item = GridElement<T>;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.values().into_iter()
        }
    }

    impl<T: Copy> Sparse2d<T> {
        /// Do these x, y coordinates exist?
        pub fn contains(&self, x: i64, y: i64) -> bool {
            if self.contains_row(y) {
                if self.get_unwrapped_row(y).contains_key(&x) {
                    return true;
                }
            }
            false
        }

        /// Does that row exist?
        pub fn contains_row(&self, y: i64) -> bool {
            self.data.contains_key(&y)
        }

        /// returns the default value of the grid
        pub fn default(&self) -> T {
            self.default
        }

        /// Get a immutable cell(x, y)
        pub fn get(&self, x: i64, y: i64) -> Option<&T> {
            if self.contains_row(y) {
                let r = self.get_unwrapped_row(y);
                if r.contains_key(&x) {
                    return r.get(&x);
                }
            }
            None
        }

        /// Get a mutable version for cell(x, y)
        pub fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
            if self.contains_row(y) {
                let r = self.get_unwrapped_mut_row(y);
                if r.contains_key(&x) {
                    return r.get_mut(&x);
                }
            }
            None
        }

        /// Return a wrapped mutable row
        fn get_mut_row(&mut self, y: i64) -> Option<&mut HashMap<i64, T>> {
            self.data.get_mut(&y)
        }

        /// Returns the mutable value of the cell after inserting the default value if non-existing cell
        pub fn get_mut_or_insert_default(&mut self, x: i64, y: i64) -> &mut T {
            self.get_mut_or_insert_value(x, y, self.default())
        }

        /// Returns the mutable value of the cell after inserting a value if non-existing cell
        pub fn get_mut_or_insert_value(&mut self, x: i64, y: i64, v: T) -> &mut T {
            self.insert(x, y, v);
            self.get_unwrapped_mut(x, y)
        }

        /// Returns a mutable row, inserts a new one if non-existing
        fn get_or_insert_mut_row(&mut self, y: i64) -> &mut HashMap<i64, T> {
            self.insert_row(y);
            self.get_unwrapped_mut_row(y)
        }

        /// Returns a wrapped immutable row
        fn get_row(&self, y: i64) -> Option<&HashMap<i64, T>> {
            self.data.get(&y)
        }

        /// Returns a unwrapped immutable cell, make sure that the cell exists!
        pub fn get_unwrapped(&self, x: i64, y: i64) -> T {
            *self.get(x, y).unwrap()
        }

        /// Returns a unwrapped mutable cell, make sure that the cell exists!
        pub fn get_unwrapped_mut(&mut self, x: i64, y: i64) -> &mut T {
            self.get_mut(x, y).unwrap()
        }

        /// Returns a unwrapped mutable row, make sure that the row exists!
        fn get_unwrapped_mut_row(&mut self, y: i64) -> &mut HashMap<i64, T> {
            self.get_mut_row(y).unwrap()
        }

        /// Returns a unwrapped immutable row, make sure that the row exists!
        fn get_unwrapped_row(&self, y: i64) -> &HashMap<i64, T> {
            self.get_row(y).unwrap()
        }

        /// Inserts value into the cell x, y if there isn't any
        pub fn insert(&mut self, x: i64, y: i64, v: T) {
            let row = self.get_or_insert_mut_row(y);
            if !row.contains_key(&x) {
                row.insert(x, v);
            }
        }

        /// Inserts the default value into the cell x, y if there isn't any
        pub fn insert_default(&mut self, x: i64, y: i64) {
            self.insert(x, y, self.default());
        }

        /// inserts a row if non-existing
        fn insert_row(&mut self, y: i64) {
            if !self.contains_row(y) {
                self.data.insert(y, HashMap::new());
            }
        }

        /// creates a new 2d Grid, sets default value
        pub fn new(default: T) -> Self {
            Self {
                data: HashMap::new(),
                default,
            }
        }

        /// Replaces/Inserts value into the cell x, y
        pub fn replace(&mut self, x: i64, y: i64, value: T) {
            self.insert_row(y);
            self.get_unwrapped_mut_row(y).insert(x, value);
        }

        pub fn values(&self) -> Vec<GridElement<T>> {
            self.data.keys().map(|y| {
                let r = self.data.get(y).unwrap();
                r.keys().map(| x | GridElement {
                    x: *x,
                    y: *y,
                    v: *r.get(x).unwrap()
                })
            }).flatten().collect()
        }
    }
}

pub mod parser {
    use std::fs;

    pub fn get_input(year: usize, day: usize) -> String {
        let path = format!("./input/{}/{}{}", year, if day < 10 { "0" } else { "" }, day);
        match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => panic!("Failed to read file {} because of {}.", path, e)
        }
    }

    pub fn get_nested_string_array(year: usize, day: usize, line_seperator: &str, inline_seperator: &str, general_seperator: &str) -> Vec<Vec<Vec<String>>> {
        get_input(year, day)
            .split(line_seperator)
            .map(| s | s.split(inline_seperator)
                .map(| s | s.split(general_seperator)
                    .map(| s | s.to_string()).collect()
                ).collect()
            ).collect()
    }

    pub fn get_nested_integer_array(year: usize, day: usize, line_seperator: &str, inline_seperator: &str, general_seperator: &str) -> Vec<Vec<Vec<i64>>> {
        get_nested_string_array(year, day, line_seperator, inline_seperator, general_seperator).iter()
            .map(| v | v.iter()
                .map(| s | s.iter()
                    .map(| s | s.parse().unwrap()).collect()
                ).collect()
            ).collect()
    }
}
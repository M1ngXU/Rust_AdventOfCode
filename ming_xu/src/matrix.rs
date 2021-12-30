use std::collections::LinkedList;
use std::default::default;
use std::fmt::{Display, Formatter, Result, Debug};

pub struct Matrix<T> {
    data: Vec<T>,
    dimensions: Vec<usize>
}

impl<T: Display + Clone> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // must be 2d
        write!(f, "{}", self.to_display_string().unwrap())
    }
}

impl<T: Debug + Clone> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // must be 2d
        write!(f, "{}", self.to_debug_string().unwrap())
    }
}

impl<T: Default + Clone> Matrix<T> {
    /// Creates a new `Matrix` by setting the dimensions - the `default value` is taken from the generic type `T`.
    pub fn new_with_default(dimensions: Vec<usize>) -> Self {
        Self::new(dimensions, default())
    }
}

impl<T: Clone> Matrix<T> {
    /// Creates a new `Matrix` by setting the dimensions and the default value.
    /// ```
    /// # use ming_xu::matrix::Matrix;
    /// let matrix = Matrix::new(vec![1, 2], 5);
    /// ```
    pub fn new(dimensions: Vec<usize>, default_value: T) -> Self {
        Self {
            data: vec![ default_value; dimensions.iter().product() ],
            dimensions
        }
    }

    fn is_in_bounds(&self, coordinates: &Vec<usize>) -> bool {
        !coordinates.iter().enumerate().any(| (i, &d) | d >= self.dimensions[i])
    }

    /// Matrix.data is a 1d array - convert a dimension vec into a 1d index
    ///
    /// If s are the sizes of the dimensions and d are the requested dimensions, then the index is
    ///
    /// a * sn + dn, a being the result value of the previous loop (starting with 0)
    fn to_one_dimension(&self, coordinates: &Vec<usize>) -> usize {
        self.dimensions.iter().enumerate().fold(0, | a, (i, &l) | a * l + coordinates[i])
    }

    fn from_one_dimension(&self, i: usize) -> Vec<usize> {
        let mut index = i;
        self.dimensions.iter().rfold(&mut LinkedList::new(), | l, &d | {
            let coordinate = index % d;
            index -= coordinate;
            index /= d;
            l.push_front(coordinate);
            l
        }).into_iter().map(| c | *c).collect()
    }

    pub fn values(&self) -> &Vec<T> {
        &self.data
    }

    pub fn modify_values(&mut self, modifier: &mut impl FnMut((Vec<usize>, &T)) -> T) {
        self.data = self.data.iter().enumerate().map(| (i, v) | modifier((self.from_one_dimension(i), v))).collect();
    }

    /// Returns the positions of adjacent cells; in a 2d matrix there are 8 adjacency cells
    fn get_adjacent(&self, coordinates: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
        if !self.is_in_bounds(coordinates) {
            return None;
        }
        let mut adjacent = Vec::new();
        let mut dimension_offsets: Vec<i64> = [-1].repeat(self.dimensions.len());
        while !dimension_offsets.iter().any(| &d | d == 2) {
            if  dimension_offsets.iter().any(| &v | v != 0) {
                let new_possible_position = coordinates.iter().enumerate().map(|(i, &d)| d as i64 + dimension_offsets[i]).collect::<Vec<i64>>();
                if !new_possible_position.contains(&-1) {// && dimension_offsets.iter().filter(| &&d | d != 0).count() == 1 {
                    let new_position = new_possible_position.iter().map(|&p| p as usize).collect();
                    if self.is_in_bounds(&new_position) {
                        adjacent.push(new_position);
                    }
                }
            }

            let mut i = 0;
            dimension_offsets[i] += 1;
            // if all offsets are 0, then the 'adjacent' cell is the requested cell => continue
            while i < dimension_offsets.len() - 1 && dimension_offsets[i] == 2 {
                dimension_offsets[i] = -1;
                i += 1;
                dimension_offsets[i] += 1;
            }
        }
        Some(adjacent)
    }

    pub fn get_adjacent_values(&self, coordinates: &Vec<usize>) -> Option<Vec<&T>> {
        self.get_adjacent(coordinates).and_then(| d | Some(d.into_iter().map(| a | self.get(&a).unwrap()).collect()))
    }

    pub fn modify_adjacent_values(&mut self, coordinates: &Vec<usize>, modifier: &mut impl FnMut((Vec<usize>, &T)) -> T) {
        if let Some(a) = self.get_adjacent(coordinates) {
            a.into_iter().for_each(| c | {
                let v = self.get_mut(&c).unwrap();
                *v = modifier((c, v));
            });
        };
    }

    /// Gets a `options` with a `value` of the matrix, `None` if it's out of bounds.
    /// ```
    /// # use ming_xu::matrix::Matrix;
    /// let matrix = Matrix::new(vec![1, 2], 5);
    ///
    /// assert_eq!(matrix.get(vec![0, 0]), Some(&5));
    /// assert_eq!(matrix.get(vec![ 9, 0 ]), None);
    /// ```
    pub fn get(&self, dimensions: &Vec<usize>) -> Option<&T> {
        self.is_in_bounds(dimensions).then(|| self.data.get(self.to_one_dimension(dimensions)).unwrap())
    }

    /// Gets a `mutable` option of a `value` of the matrix, `None` if it's out of bounds.
    /// ```
    /// # use ming_xu::matrix::Matrix;
    /// let mut matrix = Matrix::new(vec![1, 2], 5);
    ///
    /// *matrix.get_mut(vec![0, 1]).unwrap() = 2;
    /// assert_eq!(matrix.get(vec![0, 1]), Some(&2));
    /// ```
    pub fn get_mut(&mut self, dimensions: &Vec<usize>) -> Option<&mut T> {
        if self.is_in_bounds(&dimensions) {
            let d = self.to_one_dimension(&dimensions);
            // since it's inside the bounds, there is always a value => &mut T is already wrapped inside Some(T)
            self.data.get_mut(d)
        } else {
            None
        }
    }

    /// Returns the dimensions of the `matrix`.
    /// ```
    /// # use ming_xu::matrix::Matrix;
    /// let matrix = Matrix::new(vec![1, 2], 5);
    ///
    /// assert_eq!(matrix.get_dimensions(), &vec![1, 2]);
    /// ```
    pub fn get_dimensions(&self) -> &Vec<usize> {
        &self.dimensions
    }

    /// only supports 2d matrices
    pub fn to_2d_array(&self) -> Option<Vec<Vec<&T>>> {
        (self.dimensions.len() == 2).then(|| {
            let mut arr = Vec::new();
            for y in 0..self.dimensions[1] {
                let mut row = Vec::new();
                for x in 0..self.dimensions[0] {
                    row.push(self.get(&vec![x, y]).unwrap())
                }
                arr.push(row);
            }
            arr
        })
    }
}

impl<T: Debug + Clone> Matrix<T> {
    pub fn to_debug_string(&self) -> Option<String> {
        if let Some(longest) = self.data.iter().map(| d | format!("{:?}", d).len()).reduce(| p, c | p.max(c)) {
            if let Some(arr) = self.to_2d_array() {
                return Some(arr.into_iter().map(| a | a.into_iter().map(| v | {
                    let s = format!("{:?}", v);
                    let pre_spaces = " ".repeat(longest - s.len());
                    format!("{}{}", pre_spaces, s)
                }).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n"));
            }
        }
        None
    }
}

impl<T: Display + Clone> Matrix<T> {
    pub fn to_display_string(&self) -> Option<String> {
        if let Some(longest) = self.data.iter().map(| d | format!("{}", d).len()).reduce(| p, c | p.max(c)) {
            if let Some(arr) = self.to_2d_array() {
                return Some(arr.into_iter().map(| a | a.into_iter().map(| v | {
                    let s = format!("{}", v);
                    let pre_spaces = " ".repeat(longest - s.len());
                    format!("{}{}", pre_spaces, s)
                }).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n"));
            }
        }
        None
    }
}
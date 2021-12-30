use std::collections::HashMap;
use std::default::default;
use std::hash::Hash;
use std::ops::Add;

pub fn get<T, N>(
    start: T,
    is_destination: &impl Fn(&T) -> Option<N>,
    get_successor_situations: &impl Fn(&T) -> Vec<T>
) -> N where
    T: Clone + Eq + Hash,
    N: Clone + Add<Output = N> + Default
{
    get_amount(start, &mut HashMap::new(), &is_destination, &get_successor_situations)
}

fn get_amount<T, N>(
    current: T,
    visited: &mut HashMap<T, N>,
    is_destination: &impl Fn(&T) -> Option<N>,
    get_successor_situations: &impl Fn(&T) -> Vec<T>
) -> N where
    T: Clone + Eq + Hash,
    N: Clone + Add<Output = N> + Default
{
    if !visited.contains_key(&current) {
        let v = {
            if let Some(d) = is_destination(&current) {
                d
            } else {
                get_successor_situations(&current).into_iter().map(| s | get_amount(s, visited, is_destination, get_successor_situations)).fold(default(), | a, s | a + s)
            }
        };
        visited.insert(current.clone(), v);
    }
    visited.get(&current).unwrap().clone()
}
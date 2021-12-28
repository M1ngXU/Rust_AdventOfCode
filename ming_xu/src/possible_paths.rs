use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;

pub fn get<T: Copy + Eq + Hash>(
    start: T,
    is_destination: &impl Fn(&T) -> bool,
    get_successor_situations: &impl Fn(&T) -> Vec<(i64, T)>
) -> u64 {
    get_amount(start, &mut HashMap::new(), &is_destination, &get_successor_situations)
}

fn get_amount<T: Copy + Eq + Hash>(
    current: T,
    visited: &mut HashMap<T, u64>,
    is_destination: &impl Fn(&T) -> bool,
    get_successor_situations: &impl Fn(&T) -> Vec<(i64, T)>
) -> u64 {
    if !visited.contains_key(&current) {
        let v = {
            if is_destination(&current) {
                1
            } else {
                get_successor_situations(&current).into_iter().map(| (_, s) | get_amount(s, visited, is_destination, get_successor_situations)).sum()
            }
        };
        visited.insert(current, v);
    }
    *visited.get(&current).unwrap()
}
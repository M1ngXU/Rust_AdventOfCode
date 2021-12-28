use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod test;

pub trait FindAllPathsNode {
    fn is_destination(&self) -> bool;
    fn get_successor_situations(&self) -> Vec<(i64, Self)> where Self: Sized;
}

pub fn get<T: Copy + Eq + Hash + FindAllPathsNode>(start: T) -> u64 {
    get_amount(start, &mut HashMap::new())
}

fn get_amount<T: Copy + Eq + Hash + FindAllPathsNode>(
    current: T,
    visited: &mut HashMap<T, u64>
) -> u64 {
    if !visited.contains_key(&current) {
        let v = {
            if current.is_destination() {
                1
            } else {
                current.get_successor_situations().into_iter().map(| (_, s) | get_amount(s, visited)).sum()
            }
        };
        visited.insert(current, v);
    }
    *visited.get(&current).unwrap()
}
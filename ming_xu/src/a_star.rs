use std::collections::{ BinaryHeap, HashSet };
use std::cmp::{ Eq, PartialEq, PartialOrd, Ordering };
use std::option::Option;
use std::hash::{ Hash, Hasher };

struct Node<'a, T: AStarNode + Copy> {
    situation: T,
    value: i64,
    predecessor: Option<&'a Node<'a, T>>
}

pub trait AStarNode {
    fn get_successor_nodes(&self) -> Vec<(Self, i64)> where Self: Sized;
    fn is_goal(&self) -> bool;
}

//<editor-fold desc="Ordering & Equals implementations">
impl<'a, T: Eq + AStarNode + Copy> Ord for Node<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}
impl<'a, T: Eq + AStarNode + Copy> PartialOrd for Node<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl<'a, T: Eq + AStarNode + Copy> PartialEq for Node<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.situation == other.situation
    }
}
impl<'a, T: Eq + AStarNode + Copy> Eq for Node<'a, T> {}
impl<'a, T: Eq + AStarNode + Hash + Copy> Hash for Node<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.situation.hash(state);
        self.value.hash(state);
    }
}
//</editor-fold>

pub fn calc<T: Eq + AStarNode + Hash + Copy>(start: T) -> Option<i64> {
    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();

    open.push(Node {
        situation: start,
        value: 0,
        predecessor: None
    });

    while let Some(current) = open.pop() {
        if current.situation.is_goal() {
            /*let mut passed = Vec::new();
            let last = current;
            return loop {
                if let Some(last) = last.predecessor {
                    passed.push((last.situation, last.value));
                } else {
                    break Some(passed);
                }
            };*/
            return Some(current.value);
        }
        if closed.contains(&current) {
            continue;
        }

        current.situation
            .get_successor_nodes().into_iter()
            .for_each(| (s, v) | open.push(Node {
                situation: s,
                value: current.value + v,
                predecessor: None
            }));

        closed.insert(current);
    }
    None
}
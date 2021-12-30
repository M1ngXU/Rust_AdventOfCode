use std::collections::{ BinaryHeap, HashMap };
use std::cmp::{ Eq, PartialEq, PartialOrd, Ordering, Ord };
use std::option::Option;
use std::ops::Add;
use std::hash::Hash;

struct Node<T, N> {
    situation: T,
    cost: N
}
//<editor-fold desc="Implementations of standard traits for Node.">
impl<T, N: Ord> Ord for Node<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl<T, N: Ord> PartialOrd for Node<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl<T, N: Eq> PartialEq for Node<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<T, N: Eq> Eq for Node<T, N> {}
//</editor-fold>

pub fn shortest_path<T, N>(
    start: (N, T),
    is_destination: impl Fn(&T) -> bool,
    get_successor_situations: impl Fn(&T) -> Vec<(N, T)>
) -> Option<(N, Vec<(N, T)>)>
where
    T: Eq + Hash + Clone,
    N: Add + Eq + Hash + Ord + PartialOrd + Add<Output = N> + Clone
{
    let mut to_be_visited = BinaryHeap::new();
    let mut situations = HashMap::new();

    situations.insert(start.clone().1, (start.clone().0, None));

    to_be_visited.push(Node {
        situation: start.1,
        cost: start.0
    });

    while let Some(current_node) = to_be_visited.pop() {
        let cost = current_node.cost;
        if cost > situations.get(&current_node.situation).unwrap().0 {
            continue;
        }

        if is_destination(&current_node.situation) {
            let mut passed = Vec::new();
            let mut c = current_node.situation;
            while let Some(t) = situations.get(&c) {
                passed.push((t.clone().0, c));
                if let Some(u) = t.clone().1 {
                    c = u;
                } else {
                    break;
                }
            }
            passed.reverse();
            return Some((cost, passed));
        }

        for (cost_to_next, successor_situation) in get_successor_situations(&current_node.situation) {
            let cost_to_node = cost.clone() + cost_to_next;
            match situations.get(&successor_situation) {
                Some(n) if (*n).0 <= cost_to_node => {},
                _ => {
                    situations.insert(successor_situation.clone(), (cost_to_node.clone(), Some(current_node.situation.clone())));
                    to_be_visited.push(Node {
                        situation: successor_situation,
                        cost: cost_to_node
                    });
                }
            }
        }
    }
    None
}
use std::collections::{ BinaryHeap, HashMap };
use std::cmp::{ Eq, PartialEq, PartialOrd, Ordering, Ord };
use std::option::Option;
use std::hash::Hash;

#[cfg(test)]
mod test;

struct Node<T> {
    situation: T,
    cost: i64
}
//<editor-fold desc="Implementations of standard traits for Node.">
impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl<T: Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl<T: Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<T: Eq> Eq for Node<T> {}
//</editor-fold>

pub fn shortest_path<T: Eq + Hash + Clone + Copy>(
    start: T,
    is_destination: impl Fn(&T) -> bool,
    get_successor_situations: impl Fn(&T) -> Vec<(i64, T)>
) -> Option<(i64, Vec<(i64, T)>)> {
    let mut to_be_visited = BinaryHeap::new();
    let mut situations = HashMap::new();

    situations.insert(start, (0, None));

    to_be_visited.push(Node {
        situation: start,
        cost: 0
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
                passed.push((t.0, c));
                if let Some(u) = t.1 {
                    c = u;
                } else {
                    break;
                }
            }
            passed.reverse();
            return Some((cost, passed));
        }

        for (cost_to_next, successor_situation) in get_successor_situations(&current_node.situation) {
            let cost_to_node = cost + cost_to_next;
            match situations.get(&successor_situation) {
                Some(n) if (*n).0 <= cost_to_node => {},
                _ => {
                    situations.insert(successor_situation, (cost_to_node, Some(current_node.situation)));
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
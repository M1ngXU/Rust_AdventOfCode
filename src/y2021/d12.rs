use ming_xu::{file_manager, possible_paths};
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Situation {
    passed_small: Vec<String>,
    used_joker: bool,
    current: String
}

pub fn run() -> (i32, i32) {
    let mut connections = HashMap::new();
    let mut insert = | n1: &str, n2: &str | {
        if n1 != "end" && n2 != "start" {
            connections.entry(n1.to_string()).or_insert(Vec::new()).push(n2.to_string());
        }
    };
    file_manager::get_string_array(2021, 12, "-").into_iter().for_each(| l | {
        insert(&l[0], &l[1]);
        insert(&l[1], &l[0]);
    });

    let is_destination = | s: &Situation | (s.current == "end".to_string()).then_some(1);
    let get_successor_situations = | s: &Situation | {
        let mut successors = Vec::new();
        for c in connections.get(&s.current).unwrap().into_iter().filter(| c | &&c.to_lowercase() != c || !s.passed_small.contains(&c) || !s.used_joker) {
            let mut passed = s.passed_small.clone();
            if &c.to_lowercase() == c {
                passed.push(c.clone());
            }
            successors.push(Situation {
                used_joker: s.used_joker || s.passed_small.contains(&c),
                passed_small: passed,
                current: c.clone()
            });
        }
        successors
    };
    let get_paths = | has_joker: bool | possible_paths::get(Situation {
        current: "start".to_string(),
        passed_small: vec![],
        used_joker: !has_joker
    }, &is_destination, &get_successor_situations);
    (get_paths(false), get_paths(true))
}
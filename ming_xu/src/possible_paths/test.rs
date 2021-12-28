use super::get;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    value: i8
}

#[test]
fn start_at_destination() {
    assert_eq!(get(Position { value: 0 }, &| _ | true, &| _ | Vec::new()), 1);
}

#[test]
fn start_next_to_destination() {
    assert_eq!(get(Position { value: -2 }, &| p | p.value == 0, &| p | match p.value {
        0 => vec![],
        -1 => vec![(0, Position{ value: -3 })],
        -2 => vec![(0, Position{ value: -1 }), (0, Position{ value: -3 })],
        -3 => vec![(0, Position{ value: 0 })],
        _ => panic!("Lol")
    }), 2);
}
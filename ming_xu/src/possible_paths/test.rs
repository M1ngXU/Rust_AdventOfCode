use super::{ get, FindAllPathsNode };

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    value: i8
}

impl FindAllPathsNode for Position {
    fn is_destination(&self) -> bool {
        self.value == 0
    }

    fn get_successor_situations(&self) -> Vec<(i64, Self)> {
        match self.value {
            0 => vec![],
            -2 => vec![(0, Self{ value: -1 }), (0, Self{ value: -3 })],
            -1 => vec![(0, Self{ value: -3 })],
            -3 => vec![(0, Self{ value: 0 })],
            _ => panic!("Lol")
        }
    }
}

#[test]
fn start_at_destination() {
    assert_eq!(get(Position { value: 0 }), 1);
}

#[test]
fn start_next_to_destination() {
    assert_eq!(get(Position { value: -2 }), 2);
}
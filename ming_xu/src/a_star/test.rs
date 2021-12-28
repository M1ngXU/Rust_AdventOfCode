use super::shortest_path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GridElement {
    x: usize,
    y: usize,
    c: i64
}

#[test]
fn y2021_d15_aoc() {
    let input = "1163751742|1381373672|2136511328|3694931569|7463417111|1319128137|1359912421|3125421639|1293138521|2311944581"
        .split("|").enumerate().map(| (y, s) | s.chars().enumerate().map(| (x, s) | GridElement{ x, y, c: s.to_string().parse().unwrap() }).collect())
        .collect::<Vec<Vec<GridElement>>>();

    assert_eq!(
        shortest_path(
            input[0][0],
            | GridElement{ x, y, .. } | *x == input[0].len() - 1 && *y == input.len() - 1,
            | GridElement{ x, y, .. } | {
                let mut next = Vec::new();
                if x > &0 {
                    next.push((input[*y][*x - 1].c, input[*y][*x - 1]));
                }
                if x < &(input[0].len() - 1) {
                    next.push((input[*y][*x + 1].c, input[*y][*x + 1]));
                }
                if y > &0 {
                    next.push((input[*y - 1][*x].c, input[*y - 1][*x]));
                }
                if y < &(input.len() - 1) {
                    next.push((input[*y + 1][*x].c, input[*y + 1][*x]));
                }
                next
            }
        ).unwrap().0, 40
    );
}
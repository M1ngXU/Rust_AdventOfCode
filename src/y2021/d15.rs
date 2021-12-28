use ming_xu::{parser, a_star::shortest_path };

pub fn run() -> (i64, usize) {
    let start =  std::time::SystemTime::now();
    let input = parser::get_input(2021, 15).lines().enumerate().map(| (y, l) | l.chars().enumerate().map(| (x, n) | (x, y, n.to_string().parse().unwrap())).collect()).collect::<Vec<Vec<(usize, usize, i64)>>>();
    let r = (shortest_path(
        input[0][0],
        | (x, y, ..) | *x == input[0].len() - 1 && *y == input.len() - 1,
        | (x, y, ..) | {
            let mut next = Vec::new();
            if x > &0 {
                next.push((input[*y][*x - 1], input[*y][*x - 1].2));
            }
            if x < &(input[0].len() - 1) {
                next.push((input[*y][*x + 1], input[*y][*x + 1].2));
            }
            if y > &0 {
                next.push((input[*y - 1][*x], input[*y - 1][*x].2));
            }
            if y < &(input.len() - 1) {
                next.push((input[*y + 1][*x], input[*y + 1][*x].2));
            }
            next
        }
    ).unwrap().0, 2874);
    println!("{}ms", start.elapsed().unwrap().as_millis());
    r
}
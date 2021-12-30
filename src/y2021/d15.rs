use ming_xu::{file_manager, a_star::shortest_path };

pub fn run() -> (i64, usize) {
    let mut input = file_manager::get_input(2021, 15).lines().enumerate().map(|(y, l) | l.chars().enumerate().map(|(x, n) | (n.to_string().parse().unwrap(), (x, y))).collect()).collect::<Vec<Vec<(i64, (usize, usize))>>>();
    input[0][0].0 = 0;
    let p1 = shortest_path(
        input[0][0],
        | (x, y) | *x == input[0].len() - 1 && *y == input.len() - 1,
        | (x, y) | {
            let mut next = Vec::new();
            if x > &0 {
                next.push(input[*y][*x - 1]);
            }
            if x < &(input[0].len() - 1) {
                next.push(input[*y][*x + 1]);
            }
            if y > &0 {
                next.push(input[*y - 1][*x]);
            }
            if y < &(input.len() - 1) {
                next.push(input[*y + 1][*x]);
            }
            next
        }
    ).unwrap();
    file_manager::write_output_as_html(2021, 15, &input.into_iter().map(|i | i.into_iter().map(|(v, (x, y)) |
        if p1.1.iter().any(| (_, (xx, yy)) | xx == &x && yy == &y) {
            format!("<span style=\"background-color: chartreuse; font-weight: bold; color: red;\">{}</span>", v)//passed_numbers[v as usize])
        } else {
            v.to_string()
        }
    ).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
    (p1.0, 2874)
}
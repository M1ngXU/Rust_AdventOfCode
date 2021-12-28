use ming_xu::{ parser, grid::Sparse };

pub fn run() -> (usize, usize) {
    let mut grid = Sparse::new(2, 0);
    let is_diagonal = | i: &Vec<Vec<i64>> | i[0][0] != i[1][0] && i[0][1] != i[1][1];
    let mut p1= usize::MAX;
    let mut input = parser::get_nested_integer_array(2021, 5, " -> ", ",");
    input.sort_by_key(| i | if is_diagonal(i) { 1 } else { 0 });
    input.iter().for_each(| i | {
        if is_diagonal(i) && p1 == usize::MAX {
            p1 = grid.count_if(| v | v >= 2);
        }
        grid.edit_line(&vec![i[0][0], i[0][1]], &vec![i[1][0], i[1][1]], | v | *v += 1);
    });
    (p1, grid.count_if(| v | v >= 2))
}